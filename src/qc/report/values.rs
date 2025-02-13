use super::{report::Report, target::Target};
use crate::result::ReportResult;

pub struct ValuesSummary {
    target: Vec<Target>,
    ignore_not_exactly_equal: bool,
}

impl ValuesSummary {
    pub fn new(target: &[Target], ignore_not_exactly_equal: bool) -> ValuesSummary {
        ValuesSummary {
            target: target.to_vec(),
            ignore_not_exactly_equal,
        }
    }
}

impl Report for ValuesSummary {
    fn validate(&self, content: &str) -> ReportResult {
        let content = content.trim();
        for target in self.target.iter() {
            if content.starts_with(&target.target) {
                if self.ignore_not_exactly_equal && target.hint.eq("Not Exactly Equal") {
                    continue;
                }
                return ReportResult::Fail(target.hint.to_string());
            }
        }
        ReportResult::Pass
    }
}
