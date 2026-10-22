use slot_retro::{ButtonMask,LibretroCore,RetroCore};
use std::path::Path;
fn main() {
    let args:Vec<String>=std::env::args().collect();
    let core_path=Path::new(&args[1]);let rom=Path::new(&args[2]);
    let root=std::env::temp_dir().join(format!("slot-extra-smoke-{}",std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let mut core=LibretroCore::open_with(core_path,&root,&root).unwrap();
    assert!(!core.gba_turbo());assert!(!core.supports_rewind());
    let loaded=core.load(rom);
    if args.get(3).is_some_and(|s|s=="invalid-arcade") {
        // FBNeo can reject outright or return its diagnostic screen.
        if loaded.is_ok() { core.run_frame(ButtonMask::default()); }
        println!("PASS arcade invalid archive handled without a crash");return;
    }
    loaded.unwrap();let mut samples=0;
    for i in 0..120 {
        core.run_frame(ButtonMask(if i%2==0 {ButtonMask::A|ButtonMask::B|ButtonMask::START} else {0}));
        samples+=core.take_audio().len();
    }
    assert_eq!(core.video_xrgb8888().len(),640*480*4);
    assert!(samples>0);let av=core.av_info();assert!(av.fps>45.0 && av.fps<65.0);assert!(av.sample_rate>8000.0);
    let state=core.serialize().unwrap();assert!(!state.is_empty());core.unserialize(&state).unwrap();
    println!("PASS {:?}: full video, {} PCM samples, {:.3}Hz, state roundtrip; no GBA turbo/rewind",rom,samples,av.fps);
}
