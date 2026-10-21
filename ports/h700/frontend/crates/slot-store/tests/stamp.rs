use std::time::{SystemTime, UNIX_EPOCH};

use slot_store::{format_stamp, parse_stamp, stamp_now};

#[test]
fn stamps_round_trip_across_leap_days() {
    for (secs, text) in [
        (0i64, "1970-01-01_00-00-00"),
        (951_782_400, "2000-02-29_00-00-00"),
        (1_709_164_800, "2024-02-29_00-00-00"),
        (1_786_286_165, "2026-08-09_14-36-05"),
    ] {
        assert_eq!(format_stamp(secs), text);
        assert_eq!(parse_stamp(text), Some(secs));
    }
}

#[test]
fn stamps_sort_lexicographically_in_time_order() {
    let secs = [
        1_786_233_600,
        1_786_233_601,
        1_786_233_660,
        1_786_320_000,
        1_798_761_600,
    ];
    let mut stamps: Vec<String> = secs.iter().map(|s| format_stamp(*s)).collect();
    let want = stamps.clone();
    stamps.sort();
    assert_eq!(stamps, want);
}

#[test]
fn a_stamp_that_is_not_a_date_does_not_parse() {
    for bad in [
        "",
        "2026-08-09",
        "2026-08-09_14-36-05.state",
        "resume",
        "2026-13-09_14-36-05",
        "2026-08-32_14-36-05",
        "2026-08-09_24-36-05",
        "20x6-08-09_14-36-05",
    ] {
        assert_eq!(parse_stamp(bad), None, "{bad} parsed as a date");
    }
}

#[test]
fn stamp_now_is_now() {
    let secs = parse_stamp(&stamp_now()).expect("stamp_now must parse");
    let real = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before the epoch")
        .as_secs() as i64;
    assert!(
        (real - secs).abs() <= 2,
        "stamp_now read {secs}, clock {real}"
    );
}
