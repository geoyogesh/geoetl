//! Physical execution for `GeoParquet` reading.
//!
//! This module implements the physical execution plan for reading `GeoParquet` files.
#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::default_trait_access)]

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use datafusion::datasource::physical_plan::{FileScanConfig, FileStream};
use datafusion::error::Result;
use datafusion::physical_plan::{
    DisplayAs, DisplayFormatType, ExecutionPlan, Partitioning, PlanProperties,
    SendableRecordBatchStream,
};
use datafusion_execution::TaskContext;
use datafusion_physical_expr::EquivalenceProperties;

use crate::file_format::GeoParquetFormatOptions;
use crate::file_source::GeoParquetOpener;

/// Physical execution plan for reading `GeoParquet` files.
#[derive(Debug, Clone)]
pub struct GeoParquetExec {
    /// File scan configuration
    config: FileScanConfig,
    /// `GeoParquet` format options
    options: GeoParquetFormatOptions,
    /// Plan properties cache
    properties: PlanProperties,
}

impl GeoParquetExec {
    pub fn new(config: FileScanConfig, options: GeoParquetFormatOptions) -> Self {
        let properties = Self::compute_properties(&config);
        Self {
            config,
            options,
            properties,
        }
    }

    fn compute_properties(config: &FileScanConfig) -> PlanProperties {
        use datafusion::physical_plan::execution_plan::{Boundedness, EmissionType};

        let eq_properties = EquivalenceProperties::new(config.file_schema.clone());
        let output_partitioning = Partitioning::UnknownPartitioning(config.file_groups.len());

        // Create plan properties
        PlanProperties::new(
            eq_properties,
            output_partitioning,
            EmissionType::Final,
            Boundedness::Bounded,
        )
    }
}

impl DisplayAs for GeoParquetExec {
    fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter) -> fmt::Result {
        match t {
            DisplayFormatType::Default | DisplayFormatType::Verbose => {
                write!(
                    f,
                    "GeoParquetExec: file_groups={{{}}} files, projection={{{}}}",
                    self.config.file_groups.len(),
                    self.config
                        .projection
                        .as_ref()
                        .map(|p| format!("{:?}", p))
                        .unwrap_or_else(|| "all columns".to_string()),
                )
            },
            DisplayFormatType::TreeRender => {
                write!(f, "GeoParquetExec")
            },
        }
    }
}

impl ExecutionPlan for GeoParquetExec {
    fn name(&self) -> &str {
        "GeoParquetExec"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn properties(&self) -> &PlanProperties {
        &self.properties
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }

    fn with_new_children(
        self: Arc<Self>,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(self)
    }

    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let object_store_url = self.config.object_store_url.clone();
        let object_store = context.runtime_env().object_store(&object_store_url)?;

        let opener = GeoParquetOpener::new(
            self.options.clone(),
            self.config.file_schema.clone(),
            self.config.projection.clone(),
            object_store,
        );

        let stream = FileStream::new(
            &self.config,
            partition,
            Arc::new(opener),
            &Default::default(),
        )?;

        Ok(Box::pin(stream))
    }
}
