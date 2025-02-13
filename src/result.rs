use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum ReportResult {
    Pass,
    Unknown,
    Fail(String),
}

impl ReportResult {
    pub fn is_pass(&self) -> bool {
        match self {
            ReportResult::Pass => true,
            _ => false,
        }
    }
}
