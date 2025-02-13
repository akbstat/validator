use super::{
    dataset::DatasetSummary, observation::ObservationSummary, target::TargetConfig,
    values::ValuesSummary, variable::VariableSummary,
};
use crate::result::ReportResult;
use lazy_static::lazy_static;
use std::{env, path::Path};

lazy_static! {
    static ref TARGETS: TargetConfig = TargetConfig::new(Path::new(&format!(
        r"{}\qc.json",
        env::var("MK_VALIDATOR").unwrap()
    )))
    .unwrap();
}

pub trait Report {
    fn validate(&self, content: &str) -> ReportResult;
}

#[derive(Debug, PartialEq)]
pub enum ReportType {
    Dataset,
    Observation,
    Variable,
    Values,
}

impl ReportType {
    pub fn new(content: &str) -> Option<ReportType> {
        match content.trim() {
            "Data Set Summary" => Some(ReportType::Dataset),
            "数据集汇总" => Some(ReportType::Dataset),
            "Observation Summary" => Some(ReportType::Observation),
            "观测汇总" => Some(ReportType::Observation),
            "Variables Summary" => Some(ReportType::Variable),
            "变量汇总" => Some(ReportType::Variable),
            "Values Comparison Summary" => Some(ReportType::Values),
            "值比较汇总" => Some(ReportType::Values),
            _ => None,
        }
    }

    pub fn report(&self, ignore: &[String]) -> Box<dyn Report> {
        let not_exactly_equal = "Not Excatly Equal".to_string();
        let attribute_difference = "Attribute Difference".to_string();
        match self {
            ReportType::Dataset => Box::new(DatasetSummary::new()),
            ReportType::Observation => Box::new(ObservationSummary::new(&TARGETS.observation)),
            ReportType::Variable => Box::new(VariableSummary::new(
                &TARGETS.variable,
                ignore.contains(&attribute_difference),
            )),
            ReportType::Values => Box::new(ValuesSummary::new(
                &TARGETS.values,
                ignore.contains(&not_exactly_equal),
            )),
        }
    }
}
