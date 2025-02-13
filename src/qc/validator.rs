use super::{
    report::report::{Report, ReportType},
    unicode::decode_unicode,
};
use crate::result::ReportResult;
use std::{fs, ops::Sub, path::Path};

const TABLE_SYMBOL: &[u8] = br"\intbl";
const CELL_SYMBOL: &[u8] = br"\cell";
const LEFT_BRACE: u8 = b'{';

pub struct QcResultValidator {
    bytes: Vec<u8>,
    // cursor start position
    start: usize,
    // cursor end position
    end: usize,
    report: Option<Box<dyn Report>>,
    ignore: Vec<String>,
}

impl QcResultValidator {
    pub fn new<P: AsRef<Path>>(filepath: P, ignore: &[String]) -> anyhow::Result<Self> {
        let bytes: Vec<u8> = fs::read(filepath)?;
        let start = 0;
        let end = 0;
        Ok(QcResultValidator {
            bytes,
            start,
            end,
            report: None,
            ignore: ignore.to_vec(),
        })
    }

    pub fn validate(&mut self) -> anyhow::Result<ReportResult> {
        // let mut rows = 0;
        let mut report_type = None;
        while !self.end_of_file() {
            self.find_next_table();
            if self.end_of_file() {
                break;
            }
            let content = self.extract_content()?;
            // if content.eq("The COMPARE Procedure") || content.eq("COMPARE 过程") {
            //     rows = 0;
            // }
            // rows += 1;
            let imcoming_type = ReportType::new(&content);
            if report_type.ne(&imcoming_type) {
                if let Some(imcoming) = imcoming_type {
                    self.report = Some(imcoming.report(&self.ignore));
                    report_type = Some(imcoming)
                }
            }
            if let Some(report) = &self.report {
                let result = report.validate(&content);
                if !result.is_pass() {
                    return Ok(result);
                }
            }
        }
        // Ok(if rows.le(&33) {
        //     ReportResult::Pass
        // } else {
        //     ReportResult::Unknown
        // })
        Ok(ReportResult::Pass)
    }

    fn end_of_file(&self) -> bool {
        self.end.ge(&self.bytes.len()) || self.start.ge(&self.bytes.len())
    }

    /// find out next table symbol "\intbl", after finding the symbol,
    ///
    /// start cursor will be set to the start position of the symbol
    ///
    /// end cursor will be set to the end position of the symbol
    fn find_next_table(&mut self) {
        while self.end < self.bytes.len() {
            if self.end.sub(self.start).lt(&TABLE_SYMBOL.len()) {
                if self.end_of_file() {}
                self.end += 1;
                continue;
            }
            let symbol = self.bytes[self.start..self.end].to_vec();
            if symbol.eq(&TABLE_SYMBOL) {
                return;
            }
            self.start += 1;
            self.end += 1;
        }
    }

    /// extract content after finding the table symbol "\intbl", and move all cursor to the end position of the content
    fn extract_content(&mut self) -> anyhow::Result<String> {
        while self.bytes[self.start].ne(&LEFT_BRACE) {
            self.start += 1;
        }
        let content_zone_start = self.start + 1;
        self.end = self.start;
        // find out cell symbol "\cell"
        while self.end < self.bytes.len() {
            if self.end.sub(self.start).lt(&CELL_SYMBOL.len()) {
                self.end += 1;
                continue;
            }
            let symbol = self.bytes[self.start..self.end].to_vec();
            if symbol.eq(&CELL_SYMBOL) {
                break;
            }
            self.start += 1;
            self.end += 1;
        }
        let content_zone_end = self.start;
        self.start = self.end + 1;
        self.end = self.start;
        let content = String::from_utf8(self.bytes[content_zone_start..content_zone_end].to_vec())?
            .trim()
            .to_string();
        Ok(decode_unicode(&content))
    }
}

#[test]
fn test_qc_result() -> anyhow::Result<()> {
    let path: &Path = Path::new(
        r"D:\projects\rusty\mobius_kit\.mocks\qc-results\error\v-l-16-02-09-06-is-rand.rtf",
    );
    let mut qc = QcResultValidator::new(&path, &vec![])?;
    println!(
        "{:40} is pass: {:?}",
        path.file_name().unwrap().to_string_lossy(),
        qc.validate()?
    );
    Ok(())
}
