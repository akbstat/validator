use std::{env, path::Path};
use validator::sas_log::SasLogValidatior;

#[test]
fn test_log() -> anyhow::Result<()> {
    env::set_var(
        "MK_VALIDATOR",
        r"D:\projects\rusty\mobius_kit\.config\validator",
    );

    let validator = SasLogValidatior::new(None);
    let filepath = Path::new(r"D:\Studies\ak119\104\stats\dryrun\product\program\sdtm\ae.log");
    let result = validator.validate(&filepath);
    assert!(result.is_ok());
    Ok(())
}
