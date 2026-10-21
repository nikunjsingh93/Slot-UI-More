use slot::app::Phase;
use slot::session::Session;
use slot_store::{write_slot_state, Core, Platform, SlotState, StateRing};
use std::time::{Duration, Instant};

fn play(root: &std::path::Path) -> Session {
    let mut session = Session::boot(root.to_path_buf());
    let until = Instant::now() + Duration::from_secs(5);
    let mut now = 0;
    while !matches!(session.app().phase(), Phase::Playing { .. }) {
        assert!(
            Instant::now() < until,
            "SNES did not start through the frontend"
        );
        now += 16;
        session.feed([], now);
        session.update(1.0 / 60.0);
        std::thread::sleep(Duration::from_millis(2));
    }
    std::thread::sleep(Duration::from_millis(100));
    session
}
fn main() {
    std::env::set_var("SLOT_SILENT", "1");
    let args: Vec<_> = std::env::args().collect();
    let root = tempfile::tempdir().unwrap();
    slot::root::ensure(root.path());
    std::fs::copy(
        &args[1],
        root.path().join("System/snes9x2005_plus_libretro.so"),
    )
    .unwrap();
    std::fs::copy(&args[2], root.path().join("Games/SNES/Smoke.sfc")).unwrap();
    let state = SlotState {
        cart: Some("Smoke".into()),
        cart_platform: Some(Platform::Snes),
        clock_set: true,
        ..Default::default()
    };
    write_slot_state(root.path(), &state).unwrap();
    let mut session = play(root.path());
    assert_eq!(session.app().core(), Core::Snes9x2005Plus);
    session.app_mut().flush_resume();
    session.app_mut().settle_saves();
    let ram = std::fs::read(root.path().join("Saves/SNES/Smoke.sav")).unwrap();
    assert_eq!(ram.len(), 32768);
    assert_eq!(ram[0], 0x5a);
    let saved = StateRing::new(root.path(), Platform::Snes, Core::Snes9x2005Plus, "Smoke")
        .read_resume()
        .unwrap()
        .unwrap();
    assert!(!saved.is_empty());
    drop(session);
    let mut resumed = play(root.path());
    resumed.app_mut().flush_resume();
    resumed.app_mut().settle_saves();
    assert_eq!(
        std::fs::read(root.path().join("Saves/SNES/Smoke.sav")).unwrap()[0],
        0x5a
    );
    println!("PASS: real SNES core launched through Slot, saved under Saves/SNES and States/SNES, and resumed on a fresh session");
}
