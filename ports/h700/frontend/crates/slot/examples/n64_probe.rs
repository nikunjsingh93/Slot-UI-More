use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
fn main() {
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut surface = HeadlessSurface::new().unwrap();
    let core = std::env::var("SLOT_N64_CORE").unwrap();
    let rom = std::env::var("SLOT_N64_ROM").unwrap();
    let mut core = LibretroCore::open(std::path::Path::new(&core)).unwrap();
    slot::core::apply_core_options(&mut core, slot_store::Core::Mupen64plus, "", false, false);
    if let Ok(cpu) = std::env::var("SLOT_N64_CPU") {
        core.set_option("mupen64plus-cpucore", &cpu);
    }
    if std::env::var_os("SLOT_N64_NO_FB").is_some() {
        core.set_option("mupen64plus-EnableFBEmulation", "False");
        core.set_option("mupen64plus-EnableCopyColorToRDRAM", "Off");
        core.set_option("mupen64plus-EnableCopyColorFromRDRAM", "False");
    }
    println!("Options {:?}", core.options());
    std::thread::spawn(move || {
        core.load(std::path::Path::new(&rom)).unwrap();
        println!("N64 AV {:?}", core.av_info());
        let mut nonblack = 0;
        let mut audio = 0;
        for i in 0..180 {
            core.run_frame(ButtonMask(0));
            audio += core.take_audio().len();
            let v = core.video_xrgb8888();
            nonblack = v
                .chunks_exact(4)
                .filter(|p| p[0] != 0 || p[1] != 0 || p[2] != 0)
                .count();
            if i % 30 == 0 {
                println!("frame {i}: {nonblack} lit pixels, {audio} audio samples");
            }
        }
        unsafe {
            let lib = libloading::Library::new(std::env::var("SLOT_N64_CORE").unwrap()).unwrap();
            let get = lib
                .get::<unsafe extern "C" fn(u32) -> *const u8>(b"retro_get_memory_data\0")
                .unwrap();
            let size = lib
                .get::<unsafe extern "C" fn(u32) -> usize>(b"retro_get_memory_size\0")
                .unwrap();
            let p = get(2);
            let n = size(2);
            println!("RAM {n} bytes");
            if n > 0x100020 && !p.is_null() {
                println!(
                    "Code {:02x?} framebuffer {:02x?}",
                    std::slice::from_raw_parts(p.add(0x400), 32),
                    std::slice::from_raw_parts(p.add(0x100000), 32)
                );
            }
        }
        assert!(nonblack > 10000, "N64 produced no picture");
        let state = core.serialize().unwrap();
        assert!(!state.is_empty());
        core.unserialize(&state).unwrap();
        println!(
            "PASS real N64 dynarec/GLES2 frame + {} byte state round trip",
            state.len()
        );
    })
    .join()
    .unwrap();
    surface.make_current().unwrap();
}
