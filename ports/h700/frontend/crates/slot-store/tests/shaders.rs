use slot_store::{read_slot_state, write_slot_state, SlotState};

#[test]
fn old_and_invalid_shader_settings_preserve_all_other_preferences() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    let old = "cart=\ncart_platform=\nbrightness=8\nblue_light=3\nvolume=35\nmuted=0\nclock_set=1\nutc_offset_min=-240\ncolour_correction=1\n";
    for extra in ["", "shader=99\n", "shader=bad\n"] {
        std::fs::write(d.path().join("Config/slot.state"), format!("{old}{extra}")).unwrap();
        let state = read_slot_state(d.path());
        assert_eq!(state.shader, 0);
        assert_eq!((state.brightness, state.blue_light, state.volume), (8, 3, 35));
        assert!(state.clock_set && state.colour_correction);
        assert_eq!(state.utc_offset_min, -240);
    }
    for shader in 0..=4 {
        let state = SlotState { shader, ..read_slot_state(d.path()) };
        write_slot_state(d.path(), &state).unwrap();
        assert_eq!(read_slot_state(d.path()), state);
    }
}
