use super::{
    result::{FailedRow, LogResult},
    target::TargetConfig,
};
use crate::result::ReportResult;
use lazy_static::lazy_static;
use std::{env, fs, path::Path};

lazy_static! {
    static ref TARGETS: TargetConfig =
        TargetConfig::new(&format!(r"{}\log.json", env::var("MK_VALIDATOR").unwrap())).unwrap();
}

pub struct SasLogValidatior {}

impl SasLogValidatior {
    pub fn new() -> Self {
        SasLogValidatior {}
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
        for target in TARGETS.data.iter() {
            if row.to_uppercase().contains(target) {
                // handle situation "上次修改时间" to pass validation
                if target.eq("修改") && row.trim().starts_with("上次修改时间") {
                    continue;
                }
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    #[test]
    fn test_sas_log_validator() -> anyhow::Result<()> {
        let root = Path::new(r"D:\Studies\ak119\104\stats\dryrun\product\program\sdtm");
        let validator = SasLogValidatior::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir()
                || (!entry.file_name().to_string_lossy().ends_with(".log"))
            {
                continue;
            }
            let filepath = entry.path();
            let result = validator.validate(&filepath);
            assert!(result.is_ok())
        }
        Ok(())
    }
}
