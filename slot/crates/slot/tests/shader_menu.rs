#![cfg(target_os = "linux")]
use slot::frontend::Frontend;
use slot_gfx::{Compositor, HeadlessSurface, OUT_H, OUT_W};
use slot_input::{Btn, InputSource, Millis, RawEvent};
use slot_power::SimPlatform;
use slot_ui::{QuickRow, QUICK_PITCH, QUICK_TOP};

struct Input(Vec<RawEvent>);
impl InputSource for Input {
    fn poll(&mut self, _: Millis) -> Vec<RawEvent> { std::mem::take(&mut self.0) }
}
fn tap(f: &mut Frontend, key: Btn, clock: &mut Millis) {
    for event in [RawEvent::Down(key), RawEvent::Up(key)] {
        *clock += 100;
        f.advance_at(&mut Input(vec![event]), *clock, 0.1);
    }
}

#[test]
fn shader_settings_fit_the_sp_menu_and_render_each_choice() {
    std::env::set_var("SLOT_DEVICE", "g350");
    let d = tempfile::tempdir().unwrap();
    slot::root::ensure(d.path());
    for stem in ["First", "Second"] {
        std::fs::write(d.path().join(format!("Games/GBA/{stem}.gba")), [0; 256]).unwrap();
    }
    slot_store::write_slot_state(d.path(), &slot_store::SlotState { clock_set: true, ..Default::default() }).unwrap();
    let surface = HeadlessSurface::new().unwrap();
    let mut c = Compositor::new(&surface).unwrap();
    let mut f = Frontend::boot(Box::new(SimPlatform::at(d.path().to_path_buf())));
    f.upload_faces(&mut c);
    let mut clock = 1000;
    tap(&mut f, Btn::Menu, &mut clock);
    for _ in 0..QuickRow::Shader.index() { tap(&mut f, Btn::Down, &mut clock); }
    assert_eq!(f.app().quick_menu(), Some(QuickRow::Shader));
    assert!(QUICK_TOP >= 0.0);
    assert!(QUICK_TOP + QUICK_PITCH * QuickRow::ALL.len() as f32 <= 427.0);
    for shader in 0..5 {
        assert_eq!(f.app().shader(), shader);
        f.render(&mut c, (640, 480));
        let frame = c.read_frame();
        let mut viewport = [0; 4];
        unsafe { gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr()); }
        assert_eq!(viewport, [0, 0, 640, 480]);
        if let Ok(dir) = std::env::var("SCRATCH_PNG_DIR") {
            let file = std::fs::File::create(format!("{dir}/g350-shader-menu-{shader}.png")).unwrap();
            let mut e = png::Encoder::new(file, OUT_W, OUT_H);
            e.set_color(png::ColorType::Rgba);
            e.set_depth(png::BitDepth::Eight);
            e.write_header().unwrap().write_image_data(&frame).unwrap();
        }
        tap(&mut f, Btn::Right, &mut clock);
    }
    std::env::remove_var("SLOT_DEVICE");
}
