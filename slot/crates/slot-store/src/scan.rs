use std::fmt;
use std::path::{Path, PathBuf};

use crate::gba::{header_code, header_title};
use crate::platform::Platform;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cart {
    pub platform: Platform,
    pub stem: String,
    pub rom: PathBuf,
    pub label: Option<PathBuf>,
    pub title: String,
    pub code: String,
    pub shell: Option<crate::ShellChoice>,
}

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Io(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Io(e)
    }
}

pub fn scan(root: &Path) -> Result<Vec<Cart>, StoreError> {
    let read = |file: &str| std::fs::read_to_string(root.join(file)).unwrap_or_default();
    let shells = crate::cart_shell::layered(
        &read(crate::CART_SHELL_FILE),
        &read(crate::LABELS_SHELL_FILE),
    );
    let mut carts = Vec::new();
    for platform in Platform::ALL {
        let dir = root.join("Games").join(platform.dir_name());
        let entries = match std::fs::read_dir(&dir) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                eprintln!("slot: scan: {}: {e}", dir.display());
                continue;
            }
        };
        // A playlist is one shelf entry, rather than a duplicate for every disc.
        let entries: Vec<_> = entries.filter_map(Result::ok).collect();
        let mut playlist_discs = std::collections::HashSet::new();
        if matches!(platform, Platform::Ps1 | Platform::Dreamcast) {
            for entry in &entries {
                let p = entry.path();
                if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("m3u")) {
                    if let Ok(text) = std::fs::read_to_string(&p) {
                        for line in text
                            .lines()
                            .map(str::trim)
                            .filter(|l| !l.is_empty() && !l.starts_with('#'))
                        {
                            let disc = dir.join(line.replace('\\', "/"));
                            if let Ok(p) = disc.canonicalize() {
                                playlist_discs.insert(p);
                            }
                        }
                    }
                }
            }
        }
        for entry in entries {
            let rom = entry.path();
            if is_hidden(&rom)
                || !rom.is_file()
                || !platform.accepts(&rom)
                || rom
                    .canonicalize()
                    .is_ok_and(|p| playlist_discs.contains(&p))
            {
                continue;
            }
            let Some(stem) = rom.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if platform == Platform::Arcade && crate::arcade::entry(stem).is_some_and(|(bios,_)|bios) { continue; }
            let label = root
                .join("Labels")
                .join(platform.dir_name())
                .join(format!("{stem}.png"));
            let (title, code) = match platform {
                Platform::Arcade => (crate::arcade::entry(stem).map(|(_,title)|title).unwrap_or(stem).to_string(),String::new()),
                Platform::Gba => (
                    header_title(&rom).unwrap_or_default(),
                    header_code(&rom).unwrap_or_default(),
                ),
                Platform::Ps1
                | Platform::Snes
                | Platform::N64
                | Platform::Dreamcast
                | Platform::Psp
                | Platform::Nes
                | Platform::MegaDrive
                | Platform::MasterSystem
                | Platform::GameGear
                | Platform::PcEngine
                | Platform::Ports => (stem.to_string(), String::new()),
                _ => (crate::gb::title(&rom).unwrap_or_default(), String::new()),
            };
            carts.push(Cart {
                platform,
                stem: stem.to_string(),
                title,
                code,
                shell: shells.get(&crate::cart_shell::key(stem)).copied(),
                label: label.is_file().then_some(label),
                rom,
            });
        }
    }
    carts.sort_by(|a, b| {
        (a.platform as u8, sort_key(&a.stem)).cmp(&(b.platform as u8, sort_key(&b.stem)))
    });
    Ok(carts)
}

pub fn sort_key(stem: &str) -> (u8, String) {
    (group_of(stem), stem.to_uppercase())
}

fn group_of(stem: &str) -> u8 {
    match stem.chars().find(|c| !c.is_whitespace()) {
        Some(c) if c.is_ascii_digit() => 0,
        Some(c) if c.is_alphabetic() => 1,
        _ => 2,
    }
}

pub fn initial(stem: &str) -> char {
    match group_of(stem) {
        1 => stem
            .chars()
            .find(|c| !c.is_whitespace())
            .and_then(|c| c.to_uppercase().next())
            .unwrap_or('#'),
        _ => '#',
    }
}

pub fn is_hidden(p: &Path) -> bool {
    p.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
}
