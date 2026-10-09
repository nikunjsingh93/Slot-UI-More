use slot_retro::LibretroCore;
use std::sync::{Mutex, MutexGuard};

static CORE_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> MutexGuard<'static, ()> {
    CORE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn dylib() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../vendor")
        .join(if cfg!(target_os = "macos") {
            "mgba_libretro.dylib"
        } else {
            "mgba_libretro.so"
        })
}

#[test]
fn an_unset_option_reads_back_as_absent() {
    let _g = lock();
    let Ok(core) = LibretroCore::open(&dylib()) else {
        eprintln!("no core available on this host, skipping");
        return;
    };
    assert_eq!(core.option("gpsp_serial"), None);
}

#[test]
fn a_set_option_reads_back() {
    let _g = lock();
    let Ok(mut core) = LibretroCore::open(&dylib()) else {
        eprintln!("no core available on this host, skipping");
        return;
    };
    core.set_option("gpsp_serial", "rfu");
    assert_eq!(core.option("gpsp_serial"), Some("rfu".to_string()));

    core.set_option("gpsp_serial", "mul_poke");
    assert_eq!(core.option("gpsp_serial"), Some("mul_poke".to_string()));
}
