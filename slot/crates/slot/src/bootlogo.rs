use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const CARD_LOGO: &str = "System/bootlogo.bmp";
const LOGO: &str = "bootlogo.bmp";
const ORIGINAL: &str = "bootlogo.baseos.bmp";
const PARTITION: &str = "boot-resource";
const MOUNT: &str = "/tmp/slot-boot-resource";
const SIZE: usize = 54 + 720 * 480 * 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Same,
    Installed,
    Invalid,
}

pub fn valid(bmp: &[u8]) -> bool {
    let u16_at = |i: usize| u16::from_le_bytes([bmp[i], bmp[i + 1]]);
    let u32_at = |i: usize| u32::from_le_bytes([bmp[i], bmp[i + 1], bmp[i + 2], bmp[i + 3]]);
    bmp.len() == SIZE
        && &bmp[0..2] == b"BM"
        && u32_at(10) == 54
        && u32_at(14) == 40
        && u32_at(18) == 720
        && u32_at(22) == 480
        && u16_at(26) == 1
        && u16_at(28) == 24
        && u32_at(30) == 0
}

pub fn install(logo: &[u8], dir: &Path) -> io::Result<Outcome> {
    if !valid(logo) {
        return Ok(Outcome::Invalid);
    }
    let current = fs::read(dir.join(LOGO)).ok();
    if current.as_deref() == Some(logo) {
        return Ok(Outcome::Same);
    }
    if let Some(current) = &current {
        if !dir.join(ORIGINAL).exists() {
            replace(dir, ORIGINAL, current)?;
        }
    }
    replace(dir, LOGO, logo)?;
    Ok(Outcome::Installed)
}

fn replace(dir: &Path, name: &str, bytes: &[u8]) -> io::Result<()> {
    let tmp = dir.join("bootlogo.tmp");
    let mut f = File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    drop(f);
    fs::rename(&tmp, dir.join(name))?;
    File::open(dir)?.sync_all()
}

pub fn refresh(card: &Path) {
    let Ok(logo) = fs::read(card.join(CARD_LOGO)) else {
        return;
    };
    if !valid(&logo) {
        eprintln!(
            "slot: bootlogo: {CARD_LOGO} is not a 720x480 24-bit bmp, leaving the boot logo alone"
        );
        return;
    }
    let Some(device) = partition(PARTITION) else {
        eprintln!("slot: bootlogo: no {PARTITION} partition");
        return;
    };
    if fs::create_dir_all(MOUNT).is_err()
        || !run("mount", &["-t", "vfat", "-o", "rw,noatime", &device, MOUNT])
    {
        eprintln!("slot: bootlogo: could not mount {device}");
        return;
    }
    match install(&logo, Path::new(MOUNT)) {
        Ok(Outcome::Installed) => eprintln!("slot: bootlogo: installed"),
        Ok(_) => {}
        Err(e) => eprintln!("slot: bootlogo: {e}"),
    }
    if !run("umount", &[MOUNT]) {
        eprintln!("slot: bootlogo: could not unmount {MOUNT}");
    }
}

fn partition(name: &str) -> Option<String> {
    for entry in fs::read_dir("/sys/class/block").ok()?.flatten() {
        let uevent = fs::read_to_string(entry.path().join("uevent")).unwrap_or_default();
        if uevent.lines().any(|l| l == format!("PARTNAME={name}")) {
            return Some(
                PathBuf::from("/dev")
                    .join(entry.file_name())
                    .display()
                    .to_string(),
            );
        }
    }
    None
}

fn run(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .is_ok_and(|s| s.success())
}
