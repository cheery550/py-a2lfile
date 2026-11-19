//! End-to-end behavior of the `a2l-raw` bridge binary.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const MODULE: &str = r#"ASAP2_VERSION 1 71
/begin PROJECT P ""
  /begin MODULE M ""
    /begin MEASUREMENT veh_speed "自车速度" FLOAT32_IEEE NO_COMPU_METHOD 1 0 0 300
      ECU_ADDRESS 0x1000
      PHYS_UNIT "°C"
    /end MEASUREMENT
  /end MODULE
/end PROJECT
"#;

fn workdir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn run(input: &Path) -> (Output, PathBuf) {
    let output = input.with_extension("json");
    let _ = fs::remove_file(&output);
    let result = Command::new(env!("CARGO_BIN_EXE_a2l-raw"))
        .arg(input)
        .arg(&output)
        .output()
        .unwrap();
    (result, output)
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn json(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn emits_utf8_text_unchanged() {
    let dir = workdir("utf8");
    let input = dir.join("m.a2l");
    fs::write(&input, MODULE).unwrap();

    let (result, output) = run(&input);
    assert!(result.status.success(), "{}", stderr(&result));
    let model = json(&output);
    assert_eq!(model["name"], "M");
    assert_eq!(model["measurement"][0]["longIdentifier"], "自车速度");
    assert_eq!(model["measurement"][0]["phys_unit"]["unit"], "°C");
}

#[test]
fn accepts_utf8_bom() {
    let dir = workdir("bom");
    let input = dir.join("m.a2l");
    let mut bytes = b"\xEF\xBB\xBF".to_vec();
    bytes.extend_from_slice(MODULE.as_bytes());
    fs::write(&input, bytes).unwrap();

    let (result, output) = run(&input);
    assert!(result.status.success(), "{}", stderr(&result));
    assert_eq!(json(&output)["name"], "M");
}

#[test]
fn rejects_invalid_utf8_with_line_number() {
    let dir = workdir("invalid");
    let input = dir.join("m.a2l");
    // GBK bytes for "自车" on line 4: valid latin-1, invalid UTF-8.
    let (head, tail) = MODULE.split_once("自车速度").unwrap();
    let mut bytes = head.as_bytes().to_vec();
    bytes.extend_from_slice(b"\xD7\xD4\xB3\xB5");
    bytes.extend_from_slice(tail.as_bytes());
    fs::write(&input, bytes).unwrap();

    let (result, output) = run(&input);
    assert!(!result.status.success());
    assert!(!output.exists(), "no JSON may be written for rejected input");
    let message = stderr(&result);
    assert!(message.contains(":4:"), "{message}");
    assert!(message.contains("UTF-8"), "{message}");
}

#[test]
fn rejects_invalid_utf8_in_included_file() {
    let dir = workdir("include");
    fs::write(
        dir.join("part.a2l"),
        b"/begin MEASUREMENT m2 \"x\xFFy\" UBYTE NO_COMPU_METHOD 1 0 0 1\n  ECU_ADDRESS 0x2000\n/end MEASUREMENT\n",
    )
    .unwrap();
    let input = dir.join("main.a2l");
    fs::write(
        &input,
        MODULE.replace("  /end MODULE", "    /include \"part.a2l\"\n  /end MODULE"),
    )
    .unwrap();

    let (result, _) = run(&input);
    assert!(!result.status.success());
    let message = stderr(&result);
    assert!(message.contains("part.a2l:1:"), "{message}");
}

#[test]
fn ignores_include_inside_comment() {
    let dir = workdir("commented-include");
    fs::write(dir.join("gbk.a2l"), b"\xD7\xD4").unwrap();
    let input = dir.join("main.a2l");
    fs::write(
        &input,
        MODULE.replace(
            "  /end MODULE",
            "    /* /include \"gbk.a2l\" */\n    // /include \"gbk.a2l\"\n  /end MODULE",
        ),
    )
    .unwrap();

    let (result, _) = run(&input);
    assert!(result.status.success(), "{}", stderr(&result));
}
