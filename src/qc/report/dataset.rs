use super::report::Report;
use crate::result::ReportResult;
use std::cell::Cell;

pub struct DatasetSummary {
    last_nvar: Cell<Option<usize>>,
    last_nobs: Cell<Option<usize>>,
}

impl DatasetSummary {
    pub fn new() -> DatasetSummary {
        DatasetSummary {
            last_nvar: Cell::new(None),
            last_nobs: Cell::new(None),
        }
    }
}

impl Report for DatasetSummary {
    fn validate(&self, content: &str) -> ReportResult {
        let content = content.trim();
        if content.is_empty() {
            return ReportResult::Pass;
        }
        let content = content
            .split(" ")
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>();
        if content.len().eq(&5) {
            if let Ok(nvar) = content[3].parse::<usize>() {
                if let Some(last_nvar) = self.last_nvar.get() {
                    if last_nvar.ne(&nvar) {
                        return ReportResult::Fail("NVar Difference".into());
                    }
                } else {
                    self.last_nvar.set(Some(nvar));
                }
            }
            if let Ok(nobs) = content[4].parse::<usize>() {
                if let Some(last_nobs) = self.last_nobs.get() {
                    if last_nobs.ne(&nobs) {
                        return ReportResult::Fail("NObs Difference".into());
                    }
                } else {
                    self.last_nobs.set(Some(nobs));
                }
            }
        }
        ReportResult::Pass
    }
}
