use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use slot_gfx::{FbdevSurface, Surface};

const FRAMES: usize = 600;
const IRQ_NAME: &str = "dispaly";

fn irq_count() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/interrupts").ok()?;
    let line = text.lines().find(|l| l.contains(IRQ_NAME))?;
    let mut fields = line.split_whitespace();
    fields.next();
    Some(fields.take(4).filter_map(|f| f.parse::<u64>().ok()).sum())
}

fn report(name: &str, mut ms: Vec<f64>) {
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |p: f64| ms[((ms.len() - 1) as f64 * p) as usize];
    let mean = ms.iter().sum::<f64>() / ms.len() as f64;
    println!(
        "{name:>14}: mean {mean:6.2}  p5 {:6.2}  p50 {:6.2}  p95 {:6.2}  max {:6.2} ms",
        at(0.05),
        at(0.5),
        at(0.95),
        ms[ms.len() - 1]
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pace = args.first().cloned().unwrap_or_else(|| "swap".into());
    let finish = args.iter().any(|a| a == "finish");
    let delay = args
        .iter()
        .find_map(|a| a.parse::<f64>().ok())
        .unwrap_or(0.0);

    let epoch = Instant::now();
    let vsyncs = Arc::new(Mutex::new(Vec::<f64>::new()));
    let stop = Arc::new(AtomicBool::new(false));
    let watcher = {
        let (vsyncs, stop) = (vsyncs.clone(), stop.clone());
        std::thread::spawn(move || {
            let mut last = irq_count().unwrap_or(0);
            while !stop.load(Ordering::Relaxed) {
                if let Some(n) = irq_count() {
                    if n != last {
                        vsyncs
                            .lock()
                            .unwrap()
                            .push(epoch.elapsed().as_secs_f64() * 1e3);
                        last = n;
                    }
                }
                std::thread::sleep(Duration::from_micros(100));
            }
        })
    };

    let mut surface = FbdevSurface::new().expect("egl");
    gl::load_with(|s| surface.proc_address(s));
    let mut frames = Vec::new();
    let frame = Duration::from_micros(16_667);
    for i in 0..FRAMES {
        let began = Instant::now();
        if delay > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(delay / 1e3));
        }
        let c = (i % 2) as f32 * 0.2;
        unsafe {
            gl::ClearColor(c, c, c, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        let called = epoch.elapsed().as_secs_f64() * 1e3;
        surface.swap().expect("swap");
        if finish {
            unsafe { gl::Finish() };
        }
        let returned = epoch.elapsed().as_secs_f64() * 1e3;
        frames.push((called, returned));
        if pace == "sleep" {
            if let Some(left) = frame.checked_sub(began.elapsed()) {
                std::thread::sleep(left);
            }
        }
    }
    stop.store(true, Ordering::Relaxed);
    watcher.join().unwrap();
    let v = vsyncs.lock().unwrap().clone();
    let gaps: Vec<f64> = v.windows(2).map(|w| w[1] - w[0]).collect();

    let (mut wait, mut block, mut before) = (Vec::new(), Vec::new(), 0);
    for &(called, returned) in frames.iter().skip(30) {
        let Some(&next) = v.iter().find(|&&t| t > called) else {
            continue;
        };
        wait.push(next - called);
        block.push(returned - called);
        if returned < next {
            before += 1;
        }
    }
    println!(
        "pace {pace} finish {finish} delay {delay} ms; {} vsyncs seen",
        v.len()
    );
    report("vsync period", gaps);
    report("swap blocks", block);
    report("call>vsync", wait.clone());
    println!(
        "swap returned before the next vsync on {before}/{} frames",
        wait.len()
    );
}
