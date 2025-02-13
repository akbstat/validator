# Validator

> A tool for validate if log or compare result is pass or not

# Usage

1. First, you should set a environment variable named "MK_VALIDATOR" to the directory of the validation json files

```bash
$ export MK_VALIDATOR=<your folder path>
```

2. validate qc result files

```rust
#[test]
fn test_qc_result() -> anyhow::Result<()> {
    let ignore: Vec<String> = vec![];
    // ignore this if you have been set the environment variable MK_VALIDATOR
    env::set_var(
        "MK_VALIDATOR",
        r"<your folder path>",
    );

    let pass_target = Path::new(
        r"<your target qc file path>",
    );
    let mut qc = QcResultValidator::new(&pass_target, &ignore)?;
    assert_eq!(qc.validate()?, ReportResult::Pass);

    let failed_target = Path::new(
        r"<your target qc file path>",
    );
    let mut qc = QcResultValidator::new(&failed_target, &ignore)?;
    assert_eq!(
        qc.validate()?,
        ReportResult::Fail("Attribute Difference".into())
    );
    Ok(())
}
```

3. validate log file
```rust
#[test]
fn test_log() -> anyhow::Result<()> {
    // ignore this if you have been set the environment variable MK_VALIDATOR
    env::set_var(
        "MK_VALIDATOR",
        r"D:\projects\rusty\mobius_kit\.config\validator",
    );

    let validator = SasLogValidatior::new();
    let filepath = Path::new(r"<your target log file path>");
    let result = validator.validate(&filepath);
    assert!(result.is_ok());
    Ok(())
}

```

