//! `GeoJSON` parsing helpers shared across format and execution layers.
#![allow(clippy::result_large_err)]

use std::convert::TryInto;
use std::fmt;

use datafusion_shared::{SourcePosition, SpatialFormatReadError, SpatialFormatResult};
use geo_types::Geometry;
use geojson::{
    Feature, FeatureCollection, GeoJson, Geometry as GeoJsonGeometry, JsonObject, JsonValue,
};

/// Parsed `GeoJSON` feature with materialized properties and geometry.
#[derive(Debug, Clone)]
pub struct FeatureRecord {
    pub properties: JsonObject,
    pub geometry: Option<Geometry<f64>>,
}

/// Parse raw bytes into a vector of `FeatureRecord`s.
pub fn parse_geojson_bytes(
    bytes: &[u8],
    limit: Option<usize>,
    context: impl Into<String>,
) -> SpatialFormatResult<Vec<FeatureRecord>> {
    let context = context.into();
    let reader = std::io::Cursor::new(bytes);

    match GeoJson::from_reader(reader) {
        Ok(geojson) => geojson_to_records(geojson, limit, &context),
        Err(primary_err) => {
            let primary_err_message = primary_err.to_string();
            match parse_geojson_sequence(bytes, limit, &context) {
                Ok(records) => Ok(records),
                Err(sequence_err) => {
                    Err(combine_errors(&primary_err_message, &sequence_err, context))
                },
            }
        },
    }
}

/// Attempt to fix truncated `GeoJSON` by finding the last complete feature
/// and closing the JSON structure properly.
fn fix_truncated_geojson(text: &str) -> String {
    // Find the last complete feature by looking for the last "} }," pattern
    // which indicates the end of a feature object followed by comma
    if let Some(last_feature_end) = text.rfind("} },") {
        // Cut off everything after the last complete feature
        // Include the "} }" but remove the trailing comma
        let truncated = &text[..last_feature_end + 3]; // +3 to include "} }"
        // Close the features array and the FeatureCollection object
        format!("{truncated} ] }}")
    } else {
        // If we can't find a complete feature, return empty FeatureCollection
        r#"{"type":"FeatureCollection","features":[]}"#.to_string()
    }
}

/// Parse raw bytes that may contain incomplete/truncated `GeoJSON` for schema inference.
/// This is more lenient than `parse_geojson_bytes` and will extract features even from
/// truncated `FeatureCollections`.
pub fn parse_geojson_bytes_partial(
    bytes: &[u8],
    limit: Option<usize>,
    context: impl Into<String>,
) -> SpatialFormatResult<Vec<FeatureRecord>> {
    let context = context.into();

    // First try the standard parser - if it works, great!
    if let Ok(records) = parse_geojson_bytes(bytes, limit, &context) {
        return Ok(records);
    }

    // If standard parsing fails, try to manually extract features from truncated JSON
    // Parse as raw JSON value
    let text = std::str::from_utf8(bytes).map_err(|err| SpatialFormatReadError::Parse {
        message: format!("Invalid UTF-8: {err}"),
        position: None,
        context: Some(context.clone()),
    })?;

    // Try to fix truncated JSON by closing any open structures
    let fixed_text = fix_truncated_geojson(text);

    let value: JsonValue =
        serde_json::from_str(&fixed_text).map_err(|err| SpatialFormatReadError::Parse {
            message: format!("Failed to parse JSON: {err}"),
            position: None,
            context: Some(context.clone()),
        })?;

    // Extract features array from FeatureCollection
    let features_array = if let JsonValue::Object(mut obj) = value {
        if let Some(JsonValue::Array(features)) = obj.remove("features") {
            features
        } else {
            return Err(SpatialFormatReadError::Parse {
                message: "No 'features' array found in object".to_string(),
                position: None,
                context: Some(context),
            });
        }
    } else {
        return Err(SpatialFormatReadError::Parse {
            message: "Expected object with 'features' array".to_string(),
            position: None,
            context: Some(context),
        });
    };

    // Parse each feature
    let mut records = Vec::new();
    for feature_value in features_array {
        if let Ok(feature) = serde_json::from_value::<Feature>(feature_value)
            && let Ok(record) = feature_to_record(feature)
        {
            records.push(record);

            if let Some(max) = limit
                && records.len() >= max
            {
                break;
            }
        }
        // Skip features that fail to parse - we're being lenient
    }

    if records.is_empty() {
        Err(SpatialFormatReadError::Parse {
            message: "No valid features found in partial GeoJSON".to_string(),
            position: None,
            context: Some(context),
        })
    } else {
        Ok(records)
    }
}

fn geojson_to_records(
    geojson: GeoJson,
    limit: Option<usize>,
    context: &str,
) -> SpatialFormatResult<Vec<FeatureRecord>> {
    let mut records = match geojson {
        GeoJson::FeatureCollection(collection) => feature_collection_to_records(collection)?,
        GeoJson::Feature(feature) => vec![feature_to_record(feature)?],
        GeoJson::Geometry(geometry) => {
            let geometry = convert_geometry(geometry, context)?;
            vec![FeatureRecord {
                properties: JsonObject::new(),
                geometry: Some(geometry),
            }]
        },
    };

    if let Some(max) = limit
        && records.len() > max
    {
        records.truncate(max);
    }
    Ok(records)
}

fn feature_collection_to_records(
    collection: FeatureCollection,
) -> SpatialFormatResult<Vec<FeatureRecord>> {
    collection
        .features
        .into_iter()
        .map(feature_to_record)
        .collect()
}

pub(crate) fn feature_to_record(feature: Feature) -> SpatialFormatResult<FeatureRecord> {
    let geometry = match feature.geometry {
        Some(geometry) => Some(convert_geometry(geometry, "feature")?),
        None => None,
    };

    let properties = feature.properties.unwrap_or_default();

    Ok(FeatureRecord {
        properties,
        geometry,
    })
}

fn convert_geometry(
    geometry: GeoJsonGeometry,
    context: &str,
) -> SpatialFormatResult<Geometry<f64>> {
    geometry
        .try_into()
        .map_err(|err| SpatialFormatReadError::Parse {
            message: format!("Failed to convert GeoJSON geometry: {err}"),
            position: None,
            context: Some(context.to_string()),
        })
}

fn parse_geojson_sequence(
    bytes: &[u8],
    limit: Option<usize>,
    context: &str,
) -> SpatialFormatResult<Vec<FeatureRecord>> {
    let mut records = Vec::new();
    for (line_idx, raw_line) in bytes.split(|b| *b == b'\n').enumerate() {
        let line_number = (line_idx + 1) as u64;
        let line = match std::str::from_utf8(raw_line) {
            Ok(line) => line.trim(),
            Err(err) => {
                return Err(SpatialFormatReadError::Parse {
                    message: format!("GeoJSON line is not valid UTF-8: {err}"),
                    position: Some(SourcePosition {
                        line: Some(line_number),
                        ..SourcePosition::default()
                    }),
                    context: Some(context.to_string()),
                });
            },
        };

        if line.is_empty() {
            continue;
        }

        let geojson = line
            .parse::<GeoJson>()
            .map_err(|err| SpatialFormatReadError::Parse {
                message: format!("Failed to parse GeoJSON feature: {err}"),
                position: Some(SourcePosition {
                    line: Some(line_number),
                    ..SourcePosition::default()
                }),
                context: Some(context.to_string()),
            })?;

        let mut parsed = geojson_to_records(geojson, None, context)?;
        records.append(&mut parsed);

        if let Some(max) = limit
            && records.len() >= max
        {
            records.truncate(max);
            break;
        }
    }

    if records.is_empty() {
        Err(SpatialFormatReadError::Parse {
            message: "No GeoJSON features found".to_string(),
            position: None,
            context: Some(context.to_string()),
        })
    } else {
        Ok(records)
    }
}

fn combine_errors(
    collection_err: &str,
    sequence_err: &SpatialFormatReadError,
    context: String,
) -> SpatialFormatReadError {
    let message = format!(
        "Failed to parse GeoJSON as FeatureCollection ({collection_err}); \
         also failed to parse as GeoJSON sequence: {sequence_err}"
    );
    SpatialFormatReadError::Parse {
        message,
        position: None,
        context: Some(context),
    }
}

/// Helper to describe JSON value kinds for error messages.
pub(crate) fn describe_value(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool(_) => "bool",
        JsonValue::Number(_) => "number",
        JsonValue::String(_) => "string",
        JsonValue::Array(_) => "array",
        JsonValue::Object(_) => "object",
    }
}

impl fmt::Display for FeatureRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = if self.geometry.is_some() {
            "Some(Geometry)"
        } else {
            "None"
        };
        write!(
            f,
            "FeatureRecord(properties={} keys, geometry={geom})",
            self.properties.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feature_collection() {
        let data = br#"{
  "type": "FeatureCollection",
  "features": [
    {"type":"Feature","geometry":{"type":"Point","coordinates":[1.0,2.0]},"properties":{"name":"A"}},
    {"type":"Feature","geometry":null,"properties":{"value":42}}
  ]
}"#;

        let records = parse_geojson_bytes(data, None, "test").expect("parse");
        assert_eq!(records.len(), 2);
        assert!(records[0].geometry.is_some());
        assert_eq!(records[0].properties.get("name").unwrap(), "A");
        assert!(records[1].geometry.is_none());
        assert_eq!(records[1].properties.get("value").unwrap(), 42);
    }

    #[test]
    fn parse_feature_collection_with_limit() {
        let data = br#"{
  "type": "FeatureCollection",
  "features": [
    {"type":"Feature","geometry":{"type":"Point","coordinates":[1,2]},"properties":{"id":1}},
    {"type":"Feature","geometry":{"type":"Point","coordinates":[3,4]},"properties":{"id":2}},
    {"type":"Feature","geometry":{"type":"Point","coordinates":[5,6]},"properties":{"id":3}}
  ]
}"#;

        let records = parse_geojson_bytes(data, Some(2), "test").expect("parse");
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn parse_single_feature() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[5.0,10.0]},"properties":{"city":"NYC"}}"#;

        let records = parse_geojson_bytes(data, None, "test").expect("parse");
        assert_eq!(records.len(), 1);
        assert!(records[0].geometry.is_some());
        assert_eq!(records[0].properties.get("city").unwrap(), "NYC");
    }

    #[test]
    fn parse_single_feature_without_properties() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]}}"#;

        let records = parse_geojson_bytes(data, None, "test").expect("parse");
        assert_eq!(records.len(), 1);
        assert!(records[0].geometry.is_some());
        assert!(records[0].properties.is_empty());
    }

    #[test]
    fn parse_single_geometry() {
        let data = br#"{"type":"Point","coordinates":[7.0,8.0]}"#;

        let records = parse_geojson_bytes(data, None, "test").expect("parse");
        assert_eq!(records.len(), 1);
        assert!(records[0].geometry.is_some());
        assert!(records[0].properties.is_empty());
    }

    #[test]
    fn parse_sequence() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"id":1}}
{"type":"Feature","geometry":{"type":"Point","coordinates":[1,1]},"properties":{"id":2}}"#;

        let records = parse_geojson_bytes(data, Some(1), "seq").expect("sequence");
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn parse_sequence_with_empty_lines() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"id":1}}

{"type":"Feature","geometry":{"type":"Point","coordinates":[1,1]},"properties":{"id":2}}
"#;

        let records = parse_geojson_bytes(data, None, "seq").expect("sequence");
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn parse_sequence_reaches_limit() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"id":1}}
{"type":"Feature","geometry":{"type":"Point","coordinates":[1,1]},"properties":{"id":2}}
{"type":"Feature","geometry":{"type":"Point","coordinates":[2,2]},"properties":{"id":3}}"#;

        let records = parse_geojson_bytes(data, Some(2), "seq").expect("sequence");
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn parse_sequence_with_geometry_collection() {
        let data = br#"{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"id":1}}]}
{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[1,1]},"properties":{"id":2}}]}"#;

        let records = parse_geojson_bytes(data, None, "seq").expect("sequence");
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn parse_empty_sequence_fails() {
        let data = b"\n\n\n";

        let err = parse_geojson_bytes(data, None, "empty").unwrap_err();
        match err {
            SpatialFormatReadError::Parse { message, .. } => {
                assert!(message.contains("No GeoJSON features found"));
            },
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn parse_invalid_utf8_in_sequence() {
        let mut data = Vec::from(&b"{"[..]);
        data.push(0xFF); // Invalid UTF-8
        data.extend_from_slice(b"}");

        let err = parse_geojson_bytes(&data, None, "bad_utf8").unwrap_err();
        match err {
            SpatialFormatReadError::Parse { message, .. } => {
                assert!(message.contains("not valid UTF-8"));
            },
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn parse_invalid_geojson_sequence_line() {
        let data = br#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"id":1}}
not valid json"#;

        let err = parse_geojson_bytes(data, None, "bad_json").unwrap_err();
        match err {
            SpatialFormatReadError::Parse { message, .. } => {
                assert!(message.contains("Failed to parse GeoJSON feature"));
            },
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn parse_invalid_json_combines_errors() {
        let data = b"not valid json at all";

        let err = parse_geojson_bytes(data, None, "invalid").unwrap_err();
        match err {
            SpatialFormatReadError::Parse {
                message, context, ..
            } => {
                assert!(message.contains("Failed to parse GeoJSON as FeatureCollection"));
                assert!(message.contains("also failed to parse as GeoJSON sequence"));
                assert_eq!(context.as_deref(), Some("invalid"));
            },
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn describe_value_null() {
        assert_eq!(describe_value(&JsonValue::Null), "null");
    }

    #[test]
    fn describe_value_bool() {
        assert_eq!(describe_value(&JsonValue::Bool(true)), "bool");
    }

    #[test]
    fn describe_value_number() {
        assert_eq!(describe_value(&serde_json::json!(42)), "number");
    }

    #[test]
    fn describe_value_string() {
        assert_eq!(describe_value(&JsonValue::String("test".into())), "string");
    }

    #[test]
    fn describe_value_array() {
        assert_eq!(describe_value(&serde_json::json!([])), "array");
    }

    #[test]
    fn describe_value_object() {
        assert_eq!(describe_value(&serde_json::json!({})), "object");
    }

    #[test]
    fn feature_record_display_with_geometry() {
        let record = FeatureRecord {
            properties: [("key".to_string(), JsonValue::String("value".into()))]
                .iter()
                .cloned()
                .collect(),
            geometry: Some(Geometry::Point(geo_types::Point::new(1.0, 2.0))),
        };

        let display = format!("{record}");
        assert!(display.contains("properties=1 keys"));
        assert!(display.contains("Some(Geometry)"));
    }

    #[test]
    fn feature_record_display_without_geometry() {
        let record = FeatureRecord {
            properties: JsonObject::new(),
            geometry: None,
        };

        let display = format!("{record}");
        assert!(display.contains("properties=0 keys"));
        assert!(display.contains("None"));
    }
}
