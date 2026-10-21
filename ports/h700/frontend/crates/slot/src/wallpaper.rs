use std::path::{Path, PathBuf};

pub fn pick(root: &Path, seed: u64) -> Option<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(root.join("Wallpapers"))
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| !slot_store::is_hidden(p))
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("png")))
        .collect();
    if files.is_empty() {
        return None;
    }
    files.sort();
    let i = (seed % files.len() as u64) as usize;
    Some(files.swap_remove(i))
}
