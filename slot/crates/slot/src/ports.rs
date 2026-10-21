//! A port runs in its own process, after the frontend releases hardware.
use std::io;
use std::path::Path;

pub fn port_id(text: &str) -> io::Result<&str> {
    let id = text.trim();
    let script = id.strip_prefix("script:").is_some_and(|name| {
        name.len() <= 120 && name.ends_with(".sh") && !name.starts_with('.')
            && name.chars().all(|c| c.is_ascii_alphanumeric() || " ._-()".contains(c))
    });
    if matches!(id, "nfsu2" | "gtasa" | "portmaster") || script {
        Ok(id)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "unknown port id"))
    }
}

fn runner(device: &str) -> &'static str {
    match device {
        "rgsp" | "rg35xxsp" => "System/slot-h700-port-runner.sh",
        _ => "System/slot-port-runner.sh",
    }
}

pub fn request(root: &Path, descriptor: &Path) -> io::Result<()> {
    if !matches!(std::env::var("SLOT_DEVICE").as_deref(), Ok("g350" | "rgsp" | "rg35xxsp")) {
        return Err(io::Error::new(io::ErrorKind::Unsupported, "ports require a supported device"));
    }
    if descriptor.canonicalize()?.parent() != Some(root.join("Games/PORTS").canonicalize()?.as_path()) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "port outside library"));
    }
    let text = std::fs::read_to_string(descriptor)?;
    let id = port_id(&text)?;
    let device = std::env::var("SLOT_DEVICE").unwrap_or_default();
    if !root.join(runner(&device)).is_file() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "port runtime missing"));
    }
    slot_store::atomic_write(&root.join("Config/port.request"), id.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runner_matches_the_installed_device_runtime() {
        assert_eq!(runner("g350"), "System/slot-port-runner.sh");
        for device in ["rgsp", "rg35xxsp"] {
            assert_eq!(runner(device), "System/slot-h700-port-runner.sh");
        }
    }
    #[test]
    fn descriptors_are_ids_not_commands_or_paths() {
        assert_eq!(port_id("nfsu2\n").unwrap(), "nfsu2");
        assert_eq!(port_id("script:Half-Life.sh").unwrap(), "script:Half-Life.sh");
        for bad in ["", "../gtasa", "gtasa;reboot", "nfsu2\ngtasa", "/bin/sh", "script:../bad.sh", "script:x;bad.sh"] {
            assert!(port_id(bad).is_err(), "{bad}");
        }
    }
}
