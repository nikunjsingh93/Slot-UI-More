use slot::{app::Phase, frontend::Frontend};
use slot_gfx::{Compositor, HeadlessSurface, Surface};
use slot_input::{Btn, InputSource, Millis, RawEvent};
use slot_power::SimPlatform;
use slot_store::{write_slot_state, SlotState};
use std::time::{Duration, Instant};

#[derive(Default)]
struct Input(Vec<RawEvent>);
impl InputSource for Input {
    fn poll(&mut self, _: Millis) -> Vec<RawEvent> {
        std::mem::take(&mut self.0)
    }
}
fn main() {
    std::env::set_var("SLOT_SILENT", "1");
    std::env::set_var("SLOT_TEST_GLES", "1");
    std::env::set_var("SLOT_DEVICE", "g350");
    let mut surface = HeadlessSurface::new().unwrap();
    let mut compositor = Compositor::new(&surface).unwrap();
    let args: Vec<_> = std::env::args().collect();
    let root = tempfile::tempdir().unwrap();
    slot::root::ensure(root.path());
    std::fs::create_dir_all(root.path().join("Games/N64")).unwrap();
    std::fs::copy(
        &args[1],
        root.path().join("System/parallel_n64_libretro.so"),
    )
    .unwrap();
    std::fs::copy(&args[2], root.path().join("Games/N64/Probe.n64")).unwrap();
    // Slot deliberately refuses shelf ejection in a single-game library.
    // Use two entries to match the owner's real multi-game card.
    std::fs::copy(&args[2], root.path().join("Games/N64/ZZ-Other.n64")).unwrap();
    write_slot_state(
        root.path(),
        &SlotState {
            clock_set: true,
            ..Default::default()
        },
    )
    .unwrap();
    let mut frontend = Frontend::boot(Box::new(SimPlatform::at(root.path().to_path_buf())));
    frontend.upload_faces(&mut compositor);
    frontend.drive_emulator();
    let mut input = Input::default();
    let began = Instant::now();
    let mut frame = 0;
    while began.elapsed() < Duration::from_secs(60) {
        if frame == 10 {
            input.0.push(RawEvent::Down(Btn::A));
        }
        if frame == 11 {
            input.0.push(RawEvent::Up(Btn::A));
        }
        if frame > 30 && frame % 180 == 60 {
            input.0.push(RawEvent::Down(Btn::Start));
        }
        if frame > 30 && frame % 180 == 61 {
            input.0.push(RawEvent::Up(Btn::Start));
        }
        if frame > 30 && frame % 180 == 120 {
            input.0.push(RawEvent::Down(Btn::A));
        }
        if frame > 30 && frame % 180 == 121 {
            input.0.push(RawEvent::Up(Btn::A));
        }
        frontend.advance_at(&mut input, frame * 16, 1.0 / 60.0);
        frontend.step_emulator(Duration::from_millis(16), Duration::from_millis(30));
        surface.make_current().unwrap();
        frontend.compose(&mut compositor);
        let _ = compositor.read_frame();
        if frame % 60 == 0 {
            eprintln!(
                "FRONTEND {:?}, elapsed {:?}",
                frontend.app().phase(),
                began.elapsed()
            );
        }
        frame += 1;
        std::thread::sleep(Duration::from_millis(16));
    }
    assert!(matches!(frontend.app().phase(), Phase::Playing { .. }));
    println!("PASS: N64 full frontend, art, renderer, worker and audio for 60 seconds with periodic Start/A input");
    let now = frame * 16;
    input.0.push(RawEvent::Down(Btn::Menu));
    frontend.advance_at(&mut input, now, 1.0 / 60.0);
    frontend.advance_at(&mut input, now + slot_input::MENU_HOLD_MS, 1.0 / 60.0);
    input.0.push(RawEvent::Up(Btn::Menu));
    for i in 1..180 {
        frontend.advance_at(
            &mut input,
            now + slot_input::MENU_HOLD_MS + i * 16,
            1.0 / 60.0,
        );
        frontend.step_emulator(Duration::from_millis(16), Duration::from_millis(30));
        surface.make_current().unwrap();
        frontend.compose(&mut compositor);
        std::thread::sleep(Duration::from_millis(16));
    }
    assert!(matches!(frontend.app().phase(), Phase::Shelf));
    println!("PASS: holding Menu alone returns N64 to Slot shelf");
}
