use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>>{
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\n Actual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg("test/file");
    cmd.assert().failure().stderr(predicate::str::contains("no pattern to search for"));

    Ok(())
}
