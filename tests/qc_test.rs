use std::{env, path::Path};
use validator::{qc::validator::QcResultValidator, result::ReportResult};

#[test]
fn test_qc_result() -> anyhow::Result<()> {
    env::set_var(
        "MK_VALIDATOR",
        r"D:\projects\rusty\mobius_kit\.config\validator",
    );

    let ignore: Vec<String> = vec![];

    let pass_target = Path::new(
        r"D:\projects\rusty\mobius_kit\.mocks\qc-results\104307\tfl\v-f-14-02-01-01-irrc-pfs-km-fas.rtf",
    );
    let mut qc = QcResultValidator::new(&pass_target, &ignore)?;
    assert_eq!(qc.validate()?, ReportResult::Pass);

    let failed_target = Path::new(
        r"D:\projects\rusty\mobius_kit\.mocks\qc-results\104307\tfl\v-l-16-02-01-02-sf.rtf",
    );
    let mut qc = QcResultValidator::new(&failed_target, &ignore)?;
    assert_eq!(
        qc.validate()?,
        ReportResult::Fail("Attribute Difference".into())
    );
    Ok(())
}
