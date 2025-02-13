use crate::result::ReportResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LogResult {
    pub status: ReportResult,
    pub details: Vec<FailedRow>,
}

#[derive(Debug, Serialize)]
pub struct FailedRow {
    pub row: usize,
    pub content: String,
    pub pass: bool,
}
