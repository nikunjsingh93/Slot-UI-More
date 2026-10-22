use slot_store::Platform;

#[test]
fn every_platform_has_a_directory() {
    let names: Vec<&str> = Platform::ALL.iter().map(|p| p.dir_name()).collect();
    assert_eq!(
        names,
        vec!["GBA", "GB", "GBC", "PS1", "SNES", "N64", "DC", "PSP", "PORTS", "NES", "MD", "SMS", "GG", "PCE", "ARCADE"]
    );
}

#[test]
fn every_platform_is_a_shelf_of_its_own() {
    assert_eq!(
        Platform::ALL,
        [
            Platform::Gba,
            Platform::Gb,
            Platform::Gbc,
            Platform::Ps1,
            Platform::Snes,
            Platform::N64,
            Platform::Dreamcast,
            Platform::Psp,
            Platform::Ports,
            Platform::Nes,
            Platform::MegaDrive,
            Platform::MasterSystem,
            Platform::GameGear,
            Platform::PcEngine,
            Platform::Arcade,
        ]
    );
    for (i, p) in Platform::ALL.iter().enumerate() {
        assert!(
            !Platform::ALL[..i].contains(p),
            "{p:?} is named twice in the ring, so two shelves would hold one platform"
        );
    }
}

#[test]
fn each_platform_takes_only_its_own_extensions() {
    assert!(Platform::Gba.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(!Platform::Gba.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gbc")));
    assert!(!Platform::Gb.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(Platform::Gba.accepts(std::path::Path::new("Shrek.GBA")));
}

#[test]
fn ports_accept_descriptors_but_not_binaries_or_shell_scripts() {
    use std::path::Path;
    assert!(Platform::Ports.accepts(Path::new("NFSU2.port")));
    assert!(!Platform::Ports.accepts(Path::new("NFSU2.sh")));
    assert!(!Platform::Ports.accepts(Path::new("gtasa")));
}
