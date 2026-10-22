use slot_store::{Platform, Core, scan};
#[test]
fn added_systems_scan_and_bind_their_own_cores() {
 let root=tempfile::tempdir().unwrap();
 let cases=[
  (Platform::Nes, Core::Fceumm, "Example.NES"),
  (Platform::MegaDrive, Core::GenesisPlusGx, "Example.MD"),
  (Platform::MasterSystem, Core::GenesisPlusGx, "Example.SMS"),
  (Platform::GameGear, Core::GenesisPlusGx, "Example.GG"),
  (Platform::PcEngine, Core::BeetlePceFast, "Example.PCE"),
  (Platform::Arcade, Core::Fbneo, "Example.ZIP"),
];
 for (platform,core,file) in cases {
  let games=root.path().join("Games").join(platform.dir_name());
  let labels=root.path().join("Labels").join(platform.dir_name());
  std::fs::create_dir_all(&games).unwrap(); std::fs::create_dir_all(&labels).unwrap();
  std::fs::write(games.join(file), b"not a Game Boy header").unwrap();
  std::fs::write(games.join("ignored.exe"), b"ignored").unwrap();
  std::fs::write(labels.join("Example.png"), b"label").unwrap();
  assert_eq!(Core::default_for(platform),core);
  assert!(core.runs(platform));
  assert_eq!(Core::parse(core.as_str()),Some(core));
 }
 let carts=scan(root.path()).unwrap(); assert_eq!(carts.len(),cases.len());
 for c in carts { assert_eq!(c.title,"Example"); assert!(c.label.is_some()); }
 assert_eq!(Platform::Ports as usize,8); // preserve persisted settings from 1.1.16
 assert!(!Core::Mgba.runs(Platform::Nes));
 assert!(!Core::Fbneo.runs(Platform::MegaDrive));
}

#[test]
fn arcade_bios_is_hidden_but_game_keeps_its_archive_name() {
 let root=tempfile::tempdir().unwrap();let dir=root.path().join("Games/ARCADE");std::fs::create_dir_all(&dir).unwrap();
 for name in ["neogeo.zip","sf2.zip","unknown.zip"] {std::fs::write(dir.join(name),b"not read during scanning").unwrap();}
 let carts=scan(root.path()).unwrap();assert_eq!(carts.len(),2);
 let sf=carts.iter().find(|c|c.stem=="sf2").unwrap();assert!(sf.title.starts_with("Street Fighter II"));
 assert_eq!(sf.rom.file_name().unwrap(),"sf2.zip");
}
