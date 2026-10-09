use std::io;
use std::path::Path;

pub const CONFIG_DIR: &str = "Config";

const MOVED: [&str; 5] = [
    "slot.state",
    "selected_core.ini",
    "video_mode.ini",
    "cart_shell.ini",
    "theme.txt",
];

pub fn move_config(root: &Path) -> io::Result<()> {
    let config = root.join(CONFIG_DIR);
    std::fs::create_dir_all(&config)?;
    for name in MOVED {
        let old = root.join("System").join(name);
        let new = config.join(name);
        if old.is_file() && !new.exists() {
            std::fs::rename(&old, &new)?;
        }
    }
    Ok(())
}
