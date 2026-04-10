use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
const TWO_SLIDES_PPTX: &[u8] = include_bytes!("fixtures/two-slides.pptx");

#[test]
fn cli_info_prints_presentation_metadata() {
    let path = write_temp_pptx(build_two_slide_pptx());

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&path)
        .arg("--info")
        .output()
        .expect("run pptx2html --info");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"slide_count\":2"));
    assert!(stdout.contains("\"title\":null"));

    fs::remove_file(path).expect("remove temp pptx");
}

#[test]
fn cli_single_file_conversion_writes_filtered_html() {
    let path = write_temp_pptx(build_two_slide_pptx());
    let out = temp_path("pptx2html-cli-output.html");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&path)
        .arg("--slides")
        .arg("2")
        .arg("--output")
        .arg(&out)
        .output()
        .expect("run pptx2html single-file");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let html = fs::read_to_string(&out).expect("output html should exist");
    assert!(html.contains("Slide Two"));
    assert!(!html.contains("Slide One"));

    fs::remove_file(path).expect("remove temp pptx");
    fs::remove_file(out).expect("remove html");
}

#[test]
fn cli_multi_file_conversion_writes_per_slide_outputs() {
    let path = write_temp_pptx(build_two_slide_pptx());
    let out_dir = temp_path("pptx2html-cli-multi");
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).expect("remove stale output dir");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&path)
        .arg("--format")
        .arg("multi")
        .arg("--slides")
        .arg("1,2")
        .arg("--output")
        .arg(&out_dir)
        .output()
        .expect("run pptx2html multi-file");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let slide1 = fs::read_to_string(out_dir.join("slide-1.html")).expect("slide-1.html");
    let slide2 = fs::read_to_string(out_dir.join("slide-2.html")).expect("slide-2.html");
    assert!(slide1.contains("Slide One"));
    assert!(!slide1.contains("Slide Two"));
    assert!(slide2.contains("Slide Two"));
    assert!(!slide2.contains("Slide One"));

    fs::remove_file(path).expect("remove temp pptx");
    fs::remove_dir_all(out_dir).expect("remove output dir");
}

#[test]
fn cli_reports_invalid_slide_selection_and_missing_file_errors() {
    let bad_slides = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg("missing-input.pptx")
        .arg("--slides")
        .arg("5-2")
        .output()
        .expect("run pptx2html with invalid slides");
    assert!(!bad_slides.status.success());
    assert!(stderr(&bad_slides).contains("Invalid --slides value"));

    let missing_file = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg("missing-input.pptx")
        .arg("--info")
        .output()
        .expect("run pptx2html --info missing file");
    assert!(!missing_file.status.success());
    assert!(stderr(&missing_file).contains("Failed to read presentation"));
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn temp_path(suffix: &str) -> PathBuf {
    let unique = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{unique}-{suffix}"))
}

fn write_temp_pptx(bytes: Vec<u8>) -> PathBuf {
    let path = temp_path("pptx2html-cli.pptx");
    fs::write(&path, bytes).expect("write temp pptx");
    path
}

fn build_two_slide_pptx() -> Vec<u8> {
    TWO_SLIDES_PPTX.to_vec()
}
