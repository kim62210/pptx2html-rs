use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("pptx2html-cli-{name}-{nanos}"))
}

fn write_temp_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = unique_temp_path(name).with_extension("pptx");
    fs::write(&path, bytes).expect("write temp pptx");
    path
}

#[test]
fn info_command_outputs_json_metadata() {
    let input = write_temp_file("info", include_bytes!("fixtures/single-slide.pptx"));

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--info")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("\"slide_count\":1"));
    assert!(stdout.contains("\"width_px\":960.0"));

    fs::remove_file(input).ok();
}

#[test]
fn single_file_conversion_writes_requested_output() {
    let input = write_temp_file("single", include_bytes!("fixtures/single-slide.pptx"));
    let output_path = unique_temp_path("single-output").with_extension("html");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--slides")
        .arg("1")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run cli");

    assert!(output.status.success(), "{output:?}");
    let html = fs::read_to_string(&output_path).expect("read output html");
    assert!(html.contains("Slide One"));

    fs::remove_file(input).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn multi_file_conversion_writes_per_slide_outputs() {
    let input = write_temp_file("multi", include_bytes!("fixtures/two-slides.pptx"));
    let output_dir = unique_temp_path("multi-output");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(output.status.success(), "{output:?}");
    let slide_one = fs::read_to_string(output_dir.join("slide-1.html")).expect("slide 1 html");
    let slide_two = fs::read_to_string(output_dir.join("slide-2.html")).expect("slide 2 html");
    assert!(slide_one.contains("Slide One"));
    assert!(slide_two.contains("Slide Two"));

    fs::remove_file(input).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn invalid_slide_selection_returns_nonzero_exit_code() {
    let input = write_temp_file(
        "invalid-slides",
        include_bytes!("fixtures/single-slide.pptx"),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--slides")
        .arg("3-1")
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Invalid --slides value"));

    fs::remove_file(input).ok();
}

#[test]
fn missing_input_returns_nonzero_exit_code() {
    let missing = unique_temp_path("missing").with_extension("pptx");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&missing)
        .arg("--info")
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to read presentation"));
}
