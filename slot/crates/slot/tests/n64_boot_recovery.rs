use slot::app::{App, Phase};
use slot_store::{read_slot_state, write_slot_state, Platform, SlotState};

#[test]
fn saved_n64_game_cannot_crash_loop_on_boot_even_as_the_only_game() {
    let root = tempfile::tempdir().unwrap();
    slot::root::ensure(root.path());
    std::fs::create_dir_all(root.path().join("Games/N64")).unwrap();
    std::fs::write(root.path().join("Games/N64/Re-Volt.n64"), vec![0; 32768]).unwrap();
    let state = SlotState {
        cart: Some("Re-Volt".into()),
        cart_platform: Some(Platform::N64),
        clock_set: true,
        volume: 80,
        ..Default::default()
    };
    write_slot_state(root.path(), &state).unwrap();
    let app = App::boot(root.path());
    assert!(matches!(app.phase(), Phase::Shelf));
    let state = read_slot_state(root.path());
    assert_eq!(state.cart, None);
    assert_eq!(state.cart_platform, None);
    assert_eq!(state.volume, 80);
    assert!(root.path().join("Games/N64/Re-Volt.n64").is_file());
}
