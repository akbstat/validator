use super::{
    result::{FailedRow, LogResult},
    target::TargetConfig,
};
use crate::result::ReportResult;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{env, fs, path::Path};

lazy_static! {
    static ref TARGETS: TargetConfig =
        TargetConfig::new(&format!(r"{}\log.json", env::var("MK_VALIDATOR").unwrap())).unwrap();
}

pub struct SasLogValidatior {
    target_list: Vec<String>,
    white_list: Vec<String>,
    target_pattern: Vec<Regex>,
    white_list_pattern: Vec<Regex>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLogPattern {
    white_list: Vec<String>,
    issue: Vec<String>,
}

impl SasLogValidatior {
    pub fn new(external: Option<ExternalLogPattern>) -> Self {
        let mut target_pattern = TARGETS
            .target_pattern
            .clone()
            .into_iter()
            .map(|s| Regex::new(&s.to_lowercase()))
            .filter(|r| r.is_ok())
            .into_iter()
            .map(|r| r.unwrap())
            .collect::<Vec<Regex>>();
        let mut white_list_pattern = TARGETS
            .white_list_pattern
            .clone()
            .into_iter()
            .map(|s| Regex::new(&s.to_lowercase()))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<Regex>>();
        if let Some(external) = external {
            let ExternalLogPattern { white_list, issue } = external;
            white_list.into_iter().for_each(|p| {
                if let Ok(re) = Regex::new(&p.to_lowercase()) {
                    white_list_pattern.push(re);
                }
            });
            issue.into_iter().for_each(|p| {
                if let Ok(re) = Regex::new(&p.to_lowercase()) {
                    target_pattern.push(re);
                }
            });
        }
        SasLogValidatior {
            target_list: TARGETS
                .target
                .clone()
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect(),
            target_pattern,
            white_list: TARGETS
                .white_list
                .clone()
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect(),
            white_list_pattern,
        }
    }
    pub fn validate<P: AsRef<Path>>(&self, filepath: P) -> anyhow::Result<LogResult> {
        let bytes = fs::read(filepath)?;
        let mut data = vec![];
        let mut line = vec![];

        for i in 0..bytes.len() {
            let byte = bytes[i];
            if byte.eq(&b'\n') {
                let line_string = String::from_utf8_lossy(&line).to_string();
                data.push(line_string);
                line.clear();
            } else {
                line.push(byte);
            }
        }

        let mut details = vec![];
        let mut failed_counter = 0;
        data.into_iter().enumerate().for_each(|(index, row)| {
            let pass = if self.is_row_pass(&row) {
                true
            } else {
                failed_counter += 1;
                false
            };
            details.push(FailedRow {
                row: index,
                content: row.into(),
                pass,
            });
        });
        // todo!()
        Ok(LogResult {
            status: if failed_counter.gt(&0) {
                ReportResult::Fail("".into())
            } else {
                ReportResult::Pass
            },
            details,
        })
    }

    fn is_row_pass(&self, row: &str) -> bool {
        let row = row.to_lowercase();
        if self.is_row_in_white_list(&row) {
            return true;
        }
        for target in self.target_list.iter() {
            if row.contains(target) {
                return false;
            }
        }
        for pattern in self.target_pattern.iter() {
            if let Some(_) = pattern.find(&row) {
                return false;
            }
        }
        true
    }

    fn is_row_in_white_list(&self, row: &str) -> bool {
        for target in self.white_list.iter() {
            if row.contains(target) {
                return true;
            }
        }
        for pattern in self.white_list_pattern.iter() {
            if let Some(_) = pattern.find(row) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    #[test]
    fn test_sas_log_validator() -> anyhow::Result<()> {
        unsafe {
            env::set_var(
                "MK_VALIDATOR",
                r"D:\projects\rusty\mobius_kit\.config\validator",
            );
        }
        let root = Path::new(r"D:\Studies\ak139\101\stats\draft\validation\program\sdtm");
        let validator = SasLogValidatior::new(Some(ExternalLogPattern {
            white_list: vec!["does not exist.".into()],
            issue: vec![],
        }));
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir()
                || (!entry.file_name().to_string_lossy().ends_with(".log"))
            {
                continue;
            }
            let filepath = entry.path();
            let result = validator.validate(&filepath);
            assert!(result.is_ok());
            let result = result.unwrap();
            result.details.iter().for_each(|row| {
                if !row.pass {
                    dbg!(row);
                }
            });
        }
        Ok(())
    }
}
