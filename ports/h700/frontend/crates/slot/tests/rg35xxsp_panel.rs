#![cfg(target_os = "linux")]
use slot::frontend::Frontend;
use slot_gfx::{Compositor, HeadlessSurface};
use slot_power::SimPlatform;

#[test]
fn sp_frontend_fills_its_640_by_480_panel_without_black_borders() {
    std::env::set_var("SLOT_DEVICE", "rg35xxsp");
    let d = tempfile::tempdir().unwrap();
    slot::root::ensure(d.path());
    for stem in ["First", "Second"] {
        std::fs::write(d.path().join(format!("Games/PS1/{stem}.chd")), [0; 32]).unwrap();
    }
    slot_store::write_slot_state(d.path(), &slot_store::SlotState { clock_set: true, ..Default::default() }).unwrap();
    let surface = HeadlessSurface::new().unwrap();
    let mut c = Compositor::new(&surface).unwrap();
    let mut ui = Frontend::boot(Box::new(SimPlatform::at(d.path().to_path_buf())));
    ui.upload_faces(&mut c);
    ui.render(&mut c, (640, 480));
    // Inspect the final presentation viewport: an aspect-fit shelf would leave
    // 26-pixel bands above and below the 720x480 logical canvas.
    let mut viewport = [0i32; 4];
    unsafe { gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr()); }
    assert_eq!(viewport, [0, 0, 640, 480]);
    std::env::remove_var("SLOT_DEVICE");
}
