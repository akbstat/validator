use super::{report::Report, target::Target};
use crate::result::ReportResult;

pub struct ObservationSummary {
    target: Vec<Target>,
}

impl ObservationSummary {
    pub fn new(target: &[Target]) -> ObservationSummary {
        ObservationSummary {
            target: target.to_vec(),
        }
    }
}

impl Report for ObservationSummary {
    fn validate(&self, content: &str) -> ReportResult {
        let content = content.trim();
        for target in self.target.iter() {
            if content.starts_with(&target.target) {
                return ReportResult::Fail(target.hint.to_string());
            }
        }
        ReportResult::Pass
    }
}
