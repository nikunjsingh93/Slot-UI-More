use slot_store::{Core, Platform};

#[test]
fn snes_has_its_own_scanner_and_core_and_full_console_picture() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("Games/SNES")).unwrap();
    std::fs::write(dir.path().join("Games/SNES/Example.sfc"), vec![0u8; 32768]).unwrap();
    let carts = slot_store::scan(dir.path()).unwrap();
    assert_eq!(carts.len(), 1);
    assert_eq!(carts[0].platform, Platform::Snes);
    assert_eq!(carts[0].title, "Example");
    assert_eq!(Core::default_for(Platform::Snes), Core::Snes9x2005Plus);
    assert!(!Core::Mgba.runs(Platform::Snes));
    assert!(!Core::Snes9x.runs(Platform::Ps1));
    assert_eq!(Platform::Snes.picture(), (640, 480));
}
