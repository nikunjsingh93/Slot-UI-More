use std::fs;
use std::path::Path;

use slot_power::{record_first_frame, uptime_seconds};
use tempfile::TempDir;

#[test]
fn the_uptime_is_the_first_field_of_proc_uptime() {
    assert_eq!(
        uptime_seconds("1234.56 789.01\n").as_deref(),
        Some("1234.56")
    );
    assert_eq!(uptime_seconds("2.93 16.90").as_deref(), Some("2.93"));
}

#[test]
fn nothing_usable_in_proc_uptime_is_no_answer() {
    assert_eq!(uptime_seconds(""), None);
    assert_eq!(uptime_seconds("\n"), None);
    assert_eq!(uptime_seconds("  "), None);
    assert_eq!(uptime_seconds("later 789.01"), None);
}

fn read(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

#[test]
fn the_first_frame_lands_in_the_marker_and_on_the_trace() {
    let d = TempDir::new().unwrap();
    record_first_frame(d.path(), "2.93");
    assert_eq!(read(&d.path().join("boot-first-frame")).trim(), "2.93");
    let trace = read(&d.path().join("boot-trace"));
    assert!(trace.contains("2.93 first-frame"), "{trace}");
}

#[test]
fn a_later_frame_never_overwrites_the_boots_first_one() {
    let d = TempDir::new().unwrap();
    record_first_frame(d.path(), "2.93");
    record_first_frame(d.path(), "41.70");
    assert_eq!(read(&d.path().join("boot-first-frame")).trim(), "2.93");
    let trace = read(&d.path().join("boot-trace"));
    assert!(trace.contains("2.93 first-frame"), "{trace}");
    assert!(trace.contains("41.70 first-frame-again"), "{trace}");
}

#[test]
fn a_run_directory_that_does_not_exist_is_not_a_crash() {
    let d = TempDir::new().unwrap();
    record_first_frame(&d.path().join("no/such/dir"), "2.93");
}
