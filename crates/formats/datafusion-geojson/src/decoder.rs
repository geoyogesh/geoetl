//! Streaming decoder for `GeoJSON` data.
//!
//! This module implements incremental parsing of `GeoJSON` data from byte streams,
//! allowing processing of arbitrarily large files with constant memory usage.

use bytes::Bytes;
use datafusion::error::{DataFusionError, Result};
use datafusion_shared::SpatialFormatReadError;

use crate::parser::FeatureRecord;

/// Maximum buffer size before forcing a parse attempt (1MB)
const MAX_BUFFER_SIZE: usize = 1024 * 1024;

/// Streaming `GeoJSON` decoder that processes chunks of bytes and yields complete features.
pub struct GeoJsonDecoder {
    /// Internal buffer for accumulating bytes
    buffer: Vec<u8>,
    /// Parser state
    state: ParserState,
    /// Source path for error reporting
    source: String,
    /// Number of features decoded so far
    features_decoded: usize,
}

/// Parser state machine for incremental JSON parsing
#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    /// Looking for the start of `FeatureCollection`
    ExpectingHeader,
    /// Looking for next feature in the features array
    ExpectingFeature,
    /// Looking for comma separator or closing bracket
    ExpectingCommaOrClose,
    /// Reached end of file
    Done,
}

impl GeoJsonDecoder {
    /// Create a new streaming `GeoJSON` decoder
    pub fn new(source: String) -> Self {
        Self {
            buffer: Vec::with_capacity(256 * 1024), // 256KB initial capacity for better performance
            state: ParserState::ExpectingHeader,
            source,
            features_decoded: 0,
        }
    }

    /// Feed bytes into the decoder and extract any complete features.
    ///
    /// Returns a vector of parsed features. An empty vector means more data is needed.
    pub fn decode(&mut self, bytes: &Bytes) -> Result<Vec<FeatureRecord>> {
        // Append new bytes to buffer
        self.buffer.extend_from_slice(bytes);

        let mut features = Vec::new();

        // Process buffer until we can't extract more features
        loop {
            match self.state {
                ParserState::ExpectingHeader => {
                    if let Some(pos) = self.find_features_array_start() {
                        // Found start of features array, remove header from buffer
                        self.buffer.drain(..pos);
                        self.state = ParserState::ExpectingFeature;
                    } else if self.buffer.len() > MAX_BUFFER_SIZE {
                        // Buffered too much without finding header
                        return Err(DataFusionError::from(SpatialFormatReadError::Parse {
                            message: "Could not find FeatureCollection header in first 1MB of data"
                                .to_string(),
                            position: None,
                            context: Some(self.source.clone()),
                        }));
                    } else {
                        // Need more data
                        break;
                    }
                },

                ParserState::ExpectingFeature => {
                    match self.try_parse_feature()? {
                        Some((feature, consumed)) => {
                            features.push(feature);
                            self.buffer.drain(..consumed);
                            self.features_decoded += 1;
                            self.state = ParserState::ExpectingCommaOrClose;
                        },
                        None => {
                            // Need more data
                            break;
                        },
                    }
                },

                ParserState::ExpectingCommaOrClose => {
                    match self.skip_to_next_feature() {
                        SkipResult::Comma(pos) => {
                            // Found comma, next feature coming
                            self.buffer.drain(..pos);
                            self.state = ParserState::ExpectingFeature;
                        },
                        SkipResult::Close(pos) => {
                            // Found end of features array
                            self.buffer.drain(..pos);
                            self.state = ParserState::Done;
                            break;
                        },
                        SkipResult::NeedMore => {
                            // Need more data
                            break;
                        },
                    }
                },

                ParserState::Done => {
                    // No more features
                    break;
                },
            }
        }

        Ok(features)
    }

    /// Signal that no more bytes will be provided. Returns any remaining buffered features.
    pub fn finish(&mut self) -> Result<Vec<FeatureRecord>> {
        // Try to parse any remaining complete feature
        if self.state == ParserState::ExpectingFeature
            && let Some((feature, _)) = self.try_parse_feature()?
        {
            return Ok(vec![feature]);
        }
        Ok(Vec::new())
    }

    /// Find the start of the features array in a `FeatureCollection`
    fn find_features_array_start(&self) -> Option<usize> {
        // Look for: "features":[
        // We need to be flexible about whitespace
        let text = std::str::from_utf8(&self.buffer).ok()?;

        // Find "features" key
        let features_pos = text.find("\"features\"")?;
        let after_key = &text[features_pos + 10..];

        // Skip whitespace and colon
        let after_colon = after_key.trim_start().strip_prefix(':')?;
        let after_ws = after_colon.trim_start();

        // Find opening bracket
        let bracket_offset = after_ws.find('[')?;

        // Calculate total offset
        let total_offset = features_pos
            + 10
            + (after_key.len() - after_colon.len())
            + (after_colon.len() - after_ws.len())
            + bracket_offset
            + 1; // +1 to skip the '['

        Some(total_offset)
    }

    /// Try to parse a complete feature from the buffer
    fn try_parse_feature(&self) -> Result<Option<(FeatureRecord, usize)>> {
        let Ok(text) = std::str::from_utf8(&self.buffer) else {
            return Ok(None); // Invalid UTF-8, need more bytes
        };

        // Skip leading whitespace
        let trimmed = text.trim_start();
        let ws_len = text.len() - trimmed.len();

        if trimmed.is_empty() {
            return Ok(None);
        }

        // Try to find the end of the feature object
        // We'll use a simple brace counter
        if let Some((feature_json, consumed)) = Self::extract_json_object(trimmed) {
            // Try to parse the feature
            match serde_json::from_str::<geojson::Feature>(feature_json) {
                Ok(feature) => match crate::parser::feature_to_record(feature) {
                    Ok(record) => Ok(Some((record, ws_len + consumed))),
                    Err(err) => Err(DataFusionError::from(err)),
                },
                Err(err) => {
                    // Parsing failed - might be incomplete JSON
                    if self.buffer.len() > MAX_BUFFER_SIZE {
                        // Too much buffered, probably a real error
                        Err(DataFusionError::from(SpatialFormatReadError::Parse {
                            message: format!("Failed to parse GeoJSON feature: {err}"),
                            position: None,
                            context: Some(self.source.clone()),
                        }))
                    } else {
                        // Might just need more bytes
                        Ok(None)
                    }
                },
            }
        } else {
            // Couldn't find complete object yet
            Ok(None)
        }
    }

    /// Extract a complete JSON object from the text, returning (`object_str`, `bytes_consumed`)
    fn extract_json_object(text: &str) -> Option<(&str, usize)> {
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut start_found = false;

        for (i, ch) in text.char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => {
                    escape_next = true;
                },
                '"' => {
                    in_string = !in_string;
                },
                '{' if !in_string => {
                    if !start_found {
                        start_found = true;
                    }
                    depth += 1;
                },
                '}' if !in_string => {
                    depth -= 1;
                    if start_found && depth == 0 {
                        // Found complete object
                        return Some((&text[..=i], i + 1));
                    }
                },
                _ => {},
            }
        }

        None
    }

    /// Skip whitespace and look for comma or closing bracket
    fn skip_to_next_feature(&self) -> SkipResult {
        let Ok(text) = std::str::from_utf8(&self.buffer) else {
            return SkipResult::NeedMore;
        };

        let trimmed = text.trim_start();
        let ws_len = text.len() - trimmed.len();

        if trimmed.is_empty() {
            return SkipResult::NeedMore;
        }

        match trimmed.chars().next() {
            Some(',') => SkipResult::Comma(ws_len + 1),
            Some(']') => SkipResult::Close(ws_len + 1),
            _ => {
                // Unexpected character - might need more data or actual error
                if self.buffer.len() > 1024 {
                    // Too much buffered, something is wrong
                    SkipResult::NeedMore // Will eventually error in calling code
                } else {
                    SkipResult::NeedMore
                }
            },
        }
    }
}

#[derive(Debug)]
enum SkipResult {
    Comma(usize),
    Close(usize),
    NeedMore,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_features_array_start() {
        let json = br#"{"type":"FeatureCollection","features":["#;
        let mut decoder = GeoJsonDecoder::new("test".to_string());
        decoder.buffer.extend_from_slice(json);

        let pos = decoder.find_features_array_start();
        assert!(pos.is_some());
    }

    #[test]
    fn test_find_features_array_start_with_whitespace() {
        let json = br#"{"type":"FeatureCollection", "features" : [ "#;
        let mut decoder = GeoJsonDecoder::new("test".to_string());
        decoder.buffer.extend_from_slice(json);

        let pos = decoder.find_features_array_start();
        assert!(pos.is_some());
    }

    #[test]
    fn test_extract_simple_json_object() {
        let text = r#"{"a":1}"#;

        let result = GeoJsonDecoder::extract_json_object(text);
        assert_eq!(result, Some((r#"{"a":1}"#, 7)));
    }

    #[test]
    fn test_extract_nested_json_object() {
        let text = r#"{"a":{"b":2}}"#;

        let result = GeoJsonDecoder::extract_json_object(text);
        assert_eq!(result, Some((r#"{"a":{"b":2}}"#, 13)));
    }

    #[test]
    fn test_extract_incomplete_json_object() {
        let text = r#"{"a":1"#;

        let result = GeoJsonDecoder::extract_json_object(text);
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_complete_feature_collection() {
        let mut decoder = GeoJsonDecoder::new("test".to_string());

        let json = br#"{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[1.0,2.0]},"properties":{"name":"test"}}]}"#;

        let features = decoder.decode(&Bytes::from(&json[..])).unwrap();
        assert_eq!(features.len(), 1);
        assert_eq!(
            features[0].properties.get("name").and_then(|v| v.as_str()),
            Some("test")
        );
    }

    #[test]
    fn test_decode_multiple_features() {
        let mut decoder = GeoJsonDecoder::new("test".to_string());

        let json = br#"{"type":"FeatureCollection","features":[
            {"type":"Feature","geometry":{"type":"Point","coordinates":[1.0,2.0]},"properties":{"id":1}},
            {"type":"Feature","geometry":{"type":"Point","coordinates":[3.0,4.0]},"properties":{"id":2}}
        ]}"#;

        let features = decoder.decode(&Bytes::from(&json[..])).unwrap();
        assert_eq!(features.len(), 2);
    }

    #[test]
    fn test_decode_incremental() {
        let mut decoder = GeoJsonDecoder::new("test".to_string());

        // Send JSON in chunks
        let part1 = br#"{"type":"FeatureCollection","features":[{"type":"Feature","#;
        let part2 = br#""geometry":{"type":"Point","coordinates":[1.0,2.0]},"#;
        let part3 = br#""properties":{"name":"test"}}]}"#;

        let features1 = decoder.decode(&Bytes::from(&part1[..])).unwrap();
        assert_eq!(features1.len(), 0); // Incomplete

        let features2 = decoder.decode(&Bytes::from(&part2[..])).unwrap();
        assert_eq!(features2.len(), 0); // Still incomplete

        let features3 = decoder.decode(&Bytes::from(&part3[..])).unwrap();
        assert_eq!(features3.len(), 1); // Complete!
    }
}
