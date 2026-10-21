use slot::{app::Phase, session::Session};
use slot_gfx::{HeadlessSurface, Surface};
use slot_input::Action;
use slot_store::{write_slot_state, SlotState};
use std::time::{Duration, Instant};

fn main() {
    std::env::set_var("SLOT_SILENT", "1");
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut surface = HeadlessSurface::new().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let root = tempfile::tempdir().unwrap();
    slot::root::ensure(root.path());
    std::fs::create_dir_all(root.path().join("Games/N64")).unwrap();
    std::fs::copy(
        &args[1],
        root.path().join("System/mupen64plus_next_libretro.so"),
    )
    .unwrap();
    std::fs::copy(&args[2], root.path().join("Games/N64/Probe.n64")).unwrap();
    write_slot_state(
        root.path(),
        &SlotState {
            clock_set: true,
            ..Default::default()
        },
    )
    .unwrap();
    let mut session = Session::boot(root.path().to_path_buf());
    session.set_driven(true);
    assert!(matches!(session.app().phase(), Phase::Shelf));
    session.app_mut().apply(Action::Insert);
    let began = Instant::now();
    let mut now = 0;
    let mut count = 0;
    while began.elapsed() < Duration::from_secs(15) {
        now += 16;
        session.feed([], now);
        session.update(1.0 / 60.0);
        if let Some(emu) = session.emu() {
            let _ = emu.latest_frame();
            if count % 60 == 0 {
                eprintln!(
                    "SESSION {:?}, frames {}, queue {}",
                    session.app().phase(),
                    emu.published_count(),
                    session.audio_queued()
                );
            }
        }
        count += 1;
        std::thread::sleep(Duration::from_millis(16));
    }
    assert!(matches!(session.app().phase(), Phase::Playing { .. }));
    session.app_mut().flush_resume();
    session.app_mut().settle_saves();
    assert!(session.emu().unwrap().published_count() > 300);
    drop(session);
    surface.make_current().unwrap();
    println!("PASS N64 through Session/worker/PCM for 15 seconds and persisted resume");
}
