use super::{report::Report, target::Target};
use crate::result::ReportResult;

pub struct VariableSummary {
    target: Vec<Target>,
    ignore_attribute_difference: bool,
}

impl VariableSummary {
    pub fn new(target: &[Target], ignore_attribute_difference: bool) -> VariableSummary {
        VariableSummary {
            target: target.to_vec(),
            ignore_attribute_difference,
        }
    }
}

impl Report for VariableSummary {
    fn validate(&self, content: &str) -> ReportResult {
        let content = content.trim();
        for target in self.target.iter() {
            if content.contains(&target.target) {
                if self.ignore_attribute_difference && target.hint.eq("Attribute Difference") {
                    continue;
                }
                return ReportResult::Fail(target.hint.to_string());
            }
        }
        ReportResult::Pass
    }
}
