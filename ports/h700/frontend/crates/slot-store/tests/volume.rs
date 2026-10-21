use slot_store::{read_slot_state, write_slot_state, SlotState};

#[test]
fn the_headphone_level_round_trips_apart_from_the_speaker_level() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    let s = SlotState {
        volume: 70,
        volume_hp: 20,
        ..Default::default()
    };
    write_slot_state(d.path(), &s).unwrap();
    let back = read_slot_state(d.path());
    assert_eq!((back.volume, back.volume_hp), (70, 20));
}

#[test]
fn a_state_from_before_headphones_uses_the_speaker_level_for_both() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(
        d.path().join("Config/slot.state"),
        "cart=\ncart_platform=\nbrightness=5\nblue_light=0\nvolume=35\nmuted=0\nclock_set=1\nutc_offset_min=0\n",
    )
    .unwrap();
    let s = read_slot_state(d.path());
    assert_eq!((s.volume, s.volume_hp), (35, 35));
}

#[test]
fn the_headphones_mute_round_trips_and_falls_back_to_the_speakers() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    let s = SlotState {
        muted: false,
        muted_hp: true,
        ..Default::default()
    };
    write_slot_state(d.path(), &s).unwrap();
    let back = read_slot_state(d.path());
    assert_eq!((back.muted, back.muted_hp), (false, true));

    std::fs::write(
        d.path().join("Config/slot.state"),
        "cart=\ncart_platform=\nbrightness=5\nblue_light=0\nvolume=35\nmuted=1\nclock_set=1\nutc_offset_min=0\n",
    )
    .unwrap();
    let old = read_slot_state(d.path());
    assert_eq!((old.muted, old.muted_hp), (true, true));
}
