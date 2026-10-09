use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use std::time::Instant;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut core = LibretroCore::open(std::path::Path::new(&args[1])).unwrap();
    core.load(std::path::Path::new(&args[2])).unwrap();
    for _ in 0..180 {
        core.run_frame(ButtonMask(0));
        let _ = core.take_audio();
    }
    let began = Instant::now();
    let mut samples = 0;
    let mut sizes = std::collections::BTreeMap::new();
    for _ in 0..600 {
        core.run_frame(ButtonMask(0));
        let n = core.take_audio().len();
        samples += n;
        *sizes.entry(n).or_insert(0) += 1;
    }
    println!(
        "{}: {:.3} ms/frame, {} samples, AV {:?}, batches {:?}",
        args[1],
        began.elapsed().as_secs_f64() * 1000.0 / 600.0,
        samples,
        core.av_info(),
        sizes
    );
}
