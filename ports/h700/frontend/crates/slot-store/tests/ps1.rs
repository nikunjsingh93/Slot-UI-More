use slot_store::{core_for_platform, scan, Core, Platform};

#[test]
fn playlists_hide_their_discs_and_bin_tracks() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("Games/PS1");
    std::fs::create_dir_all(&dir).unwrap();
    for name in ["Disc 1.chd", "Disc 2.cue", "Disc 2.bin", "Standalone.CHD"] {
        std::fs::write(dir.join(name), b"fixture").unwrap();
    }
    std::fs::write(
        dir.join("Adventure.m3u"),
        "# discs\r\nDisc 1.chd\r\nDisc 2.cue\r\n",
    )
    .unwrap();
    let games = scan(root.path()).unwrap();
    assert_eq!(
        games.iter().map(|c| c.stem.as_str()).collect::<Vec<_>>(),
        vec!["Adventure", "Standalone"]
    );
    assert!(games.iter().all(|c| c.platform == Platform::Ps1));
    assert_eq!(
        core_for_platform(root.path(), "Adventure", Platform::Ps1),
        Core::PcsxRearmed
    );
    assert!(!Core::Mgba.runs(Platform::Ps1));
    assert!(!Platform::Ps1.accepts(std::path::Path::new("track.bin")));
}
