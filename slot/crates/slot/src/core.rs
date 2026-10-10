use std::path::{Path, PathBuf};

use slot_retro::{LibretroCore, MockCore, RetroCore};
use slot_store::Core;

use crate::root;

const CORE_ENV: &str = "SLOT_CORE";

pub fn dylib_name(core: Core) -> String {
    format!(
        "{}_libretro.{}",
        core.as_str(),
        std::env::consts::DLL_EXTENSION
    )
}

fn candidates(root: &Path, core: Core) -> Vec<PathBuf> {
    if let Some(named) = std::env::var_os(CORE_ENV) {
        return vec![PathBuf::from(named)];
    }
    let name = dylib_name(core);
    let mut paths = vec![root.join("System").join(&name)];
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
    {
        paths.push(dir.join(&name));
        paths.push(dir.join("vendor").join(&name));
    }
    paths.push(Path::new("vendor").join(&name));
    paths
}

pub struct Opened {
    pub core: Box<dyn RetroCore>,
    pub named: bool,
}

pub fn open_core(root: &Path, core: Core, serial: &str, colour: bool, link: Option<u8>) -> Opened {
    let paths = candidates(root, core);
    match open_named(root, core, serial, colour, link, &paths) {
        Some(core) => Opened { core, named: true },
        None => {
            report_missing(core, &paths);
            Opened {
                core: Box::new(MockCore::new()),
                named: false,
            }
        }
    }
}

pub fn open_core_for(
    root: &Path,
    core: Core,
    serial: &str,
    colour: bool,
    paths: &[PathBuf],
) -> Box<dyn RetroCore> {
    open_named(root, core, serial, colour, None, paths).unwrap_or_else(|| {
        report_missing(core, paths);
        Box::new(MockCore::new())
    })
}

fn open_named(
    root: &Path,
    core: Core,
    serial: &str,
    colour: bool,
    link: Option<u8>,
    paths: &[PathBuf],
) -> Option<Box<dyn RetroCore>> {
    let bios = root::bios_dir(root);
    let saves = match core {
        Core::PcsxRearmed => root.join("Saves/PS1"),
        Core::Flycast => root.join("Saves/DC"),
        Core::Ppsspp => root.join("Saves/PSP/MemoryStick"),
        _ => root::saves_dir(root),
    };
    let _ = std::fs::create_dir_all(&saves);
    if core == Core::Ppsspp {
        for folder in ["PSP/SAVEDATA", "PSP/SYSTEM", "PSP/SYSTEM/CACHE", "PSP/GAME"] {
            let _ = std::fs::create_dir_all(saves.join(folder));
        }
    }
    for path in paths {
        if !path.exists() {
            continue;
        }
        match LibretroCore::open_with(path, &bios, &saves) {
            Ok(mut opened) => {
                apply_core_options(&mut opened, core, serial, root::has_real_bios(root), colour);
                if let Some(player) = link {
                    apply_link_options(&mut opened, core, player);
                }
                eprintln!("slot: core {}", path.display());
                return Some(Box::new(opened));
            }
            Err(e) => eprintln!("slot: {}: {e}", path.display()),
        }
    }
    None
}

fn report_missing(core: Core, paths: &[PathBuf]) {
    eprintln!(
        "slot: no {} core found, running the mock test pattern instead. Looked in: {}",
        core.as_str(),
        paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

pub fn colour_option(which: Core, on: bool) -> Option<(&'static str, &'static str)> {
    match which {
        Core::PcsxRearmed
        | Core::Snes9x
        | Core::Snes9x2005Plus
        | Core::Mupen64plus
        | Core::ParallelN64
        | Core::Flycast
        | Core::Ppsspp => None,
        Core::Mgba => Some(("mgba_color_correction", if on { "Auto" } else { "OFF" })),
        Core::Gpsp => Some((
            "gpsp_color_correction",
            if on { "enabled" } else { "disabled" },
        )),
    }
}

pub fn apply_link_options(core: &mut LibretroCore, which: Core, player: u8) {
    if which != Core::Mgba {
        return;
    }
    core.set_option("mgba_link", "on");
    core.set_option("mgba_link_player", &player.to_string());
    eprintln!("slot: core: link mode on, player {player}");
}

pub fn apply_core_options(
    core: &mut LibretroCore,
    which: Core,
    serial: &str,
    bios: bool,
    colour: bool,
) {
    if which == Core::Flycast {
        core.set_option("reicast_internal_resolution", "320x240");
        core.set_option("reicast_threaded_rendering", "enabled");
        // Same low-end renderer settings used by dArkOS's RK3326 RetroRun profile.
        core.set_option("reicast_alpha_sorting", "per-strip (fast, least accurate)");
        core.set_option("reicast_volume_modifier_enable", "disabled");
        core.set_option("reicast_enable_rttb", "disabled");
        core.set_option("reicast_delay_frame_swapping", "disabled");
        core.set_option("reicast_anisotropic_filtering", "off");
        core.set_option("reicast_texupscale", "1");
        core.set_option("reicast_enable_dsp", "disabled");
        core.set_option("reicast_hle_bios", "enabled");
        core.set_option("reicast_digital_triggers", "enabled");
        core.set_option("reicast_per_content_vmus", "VMU A1");
        // Render every other frame while retaining the emulated CPU/audio clock.
        // Maximum adaptive skipping also lets the CPU continue when Mali is busy.
        core.set_option("reicast_frame_skipping", "1");
        core.set_option("reicast_auto_skip_frame", "more");
        eprintln!("slot: Dreamcast Flycast / GLES3 / native 320x240 framebuffer / per-strip sorting / threaded renderer / frameskip 1 + adaptive / HLE BIOS");
        return;
    }
    if which == Core::Ppsspp {
        core.set_option("ppsspp_backend", "opengl");
        core.set_option("ppsspp_software_rendering", "disabled");
        core.set_option("ppsspp_cpu_core", "JIT");
        core.set_option("ppsspp_internal_resolution", "480x272");
        core.set_option("ppsspp_frameskip", "1");
        core.set_option("ppsspp_auto_frameskip", "enabled");
        core.set_option("ppsspp_texture_scaling_level", "disabled");
        core.set_option("ppsspp_texture_shader", "disabled");
        core.set_option("ppsspp_frame_duplication", "disabled");
        eprintln!("slot: PSP PPSSPP / GLES2 / native 480x272 / JIT / bounded auto frameskip");
        return;
    }
    if which == Core::ParallelN64 {
        core.set_option("parallel-n64-cpucore", "dynamic_recompiler");
        core.set_option("parallel-n64-rspplugin", "hle");
        core.set_option("parallel-n64-gfxplugin", "glide64");
        core.set_option("parallel-n64-screensize", "320x240");
        // This core spells the alternate layout differently from Mupen-Next.
        // It matches Slot's existing N64 A/B, shoulders and C-button mapping.
        core.set_option("parallel-n64-alt-map", "enabled");
        core.set_option("parallel-n64-framerate", "original");
        eprintln!("slot: N64 ParaLLEl / Glide64 GLES2 / HLE / 320x240 / original controls");
        return;
    }
    if which == Core::Mupen64plus {
        core.set_option("mupen64plus-cpucore", "dynamic_recompiler");
        core.set_option("mupen64plus-rdp-plugin", "gliden64");
        core.set_option("mupen64plus-rsp-plugin", "hle");
        core.set_option("mupen64plus-alt-map", "True");
        // A modest 4:3 reduction cuts pixel work without changing the emulator.
        core.set_option("mupen64plus-43screensize", "256x192");
        // Preserve the old native framebuffer behavior. Fractional internal
        // scaling costs more CPU in our tests and changes game compatibility.
        core.set_option("mupen64plus-EnableNativeResFactor", "1");
        core.set_option("mupen64plus-ThreadedRenderer", "False");
        core.set_option("mupen64plus-txHiresEnable", "False");
        core.set_option("mupen64plus-EnableFBEmulation", "True");
        // Synchronous readback stalls the tile-based G350 GPU each VI.
        core.set_option("mupen64plus-EnableCopyColorToRDRAM", "Off");
        core.set_option("mupen64plus-EnableCopyColorFromRDRAM", "False");
        core.set_option("mupen64plus-EnableCopyDepthToRDRAM", "Off");
        core.set_option("mupen64plus-HybridFilter", "False");
        core.set_option("mupen64plus-FrameDuping", "True");
        eprintln!(
            "slot: N64 256x192 performance viewport, framebuffer copies off, bounded readback skip"
        );
        return;
    }
    if which == Core::Snes9x {
        core.set_option("snes9x_overclock_superfx", "100%");
        core.set_option("snes9x_overclock_cycles", "disabled");
        return;
    }
    if which == Core::Snes9x2005Plus {
        core.set_option("snes9x_2005_frameskip", "auto");
        core.set_option("snes9x_2005_frameskip_threshold", "33");
        return;
    }
    if which == Core::PcsxRearmed {
        core.set_option("pcsx_rearmed_frameskip_type", "disabled");
        core.set_option("pcsx_rearmed_drc", "enabled");
        core.set_option("pcsx_rearmed_memcard1", "libretro");
        core.set_option("pcsx_rearmed_memcard2", "none");
        return;
    }
    core.set_option(&format!("{}_frameskip", which.as_str()), "auto");
    if which == Core::Mgba {
        core.set_option("mgba_sgb_borders", "OFF");
        core.set_option("mgba_gb_colors_preset", "1");
        core.set_option("mgba_gb_colors", "GBC Dark Green →A");
        if let Some((key, value)) = colour_option(which, colour) {
            core.set_option(key, value);
        }
    }
    if which == Core::Gpsp {
        core.set_option("gpsp_serial", serial);
        if bios {
            core.set_option("gpsp_boot_mode", "bios");
        }
        if let Some((key, value)) = colour_option(which, colour) {
            core.set_option(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn each_core_spells_colour_correction_its_own_way() {
        assert_eq!(
            colour_option(Core::Mgba, true),
            Some(("mgba_color_correction", "Auto"))
        );
        assert_eq!(
            colour_option(Core::Mgba, false),
            Some(("mgba_color_correction", "OFF"))
        );
        assert_eq!(
            colour_option(Core::Gpsp, true),
            Some(("gpsp_color_correction", "enabled"))
        );
        assert_eq!(
            colour_option(Core::Gpsp, false),
            Some(("gpsp_color_correction", "disabled"))
        );
    }

    fn lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn candidates_search_the_named_cores_own_filename_only() {
        let _g = lock();
        std::env::remove_var(CORE_ENV);

        let root = Path::new("/root");
        let mgba = candidates(root, Core::Mgba);
        let gpsp = candidates(root, Core::Gpsp);
        assert_ne!(mgba, gpsp, "the two cores searched the same paths");
        assert!(!mgba.is_empty());
        assert!(!gpsp.is_empty());
        for path in &mgba {
            let name = path.file_name().unwrap().to_str().unwrap();
            assert!(name.starts_with("mgba_libretro"), "{name} is not mGBA's");
        }
        for path in &gpsp {
            let name = path.file_name().unwrap().to_str().unwrap();
            assert!(name.starts_with("gpsp_libretro"), "{name} is not gpSP's");
        }
    }

    #[test]
    fn candidates_search_the_roots_own_system_directory() {
        let _g = lock();
        std::env::remove_var(CORE_ENV);
        let root = Path::new("/some/content/root");
        assert_eq!(
            candidates(root, Core::Gpsp)[0],
            root.join("System").join(dylib_name(Core::Gpsp)),
        );
    }

    #[test]
    fn the_env_override_ignores_which_core_was_asked_for() {
        let _g = lock();
        std::env::set_var(CORE_ENV, "/dev/null/named-core");
        let root = Path::new("/root");
        let mgba = candidates(root, Core::Mgba);
        let gpsp = candidates(root, Core::Gpsp);
        std::env::remove_var(CORE_ENV);
        assert_eq!(mgba, vec![PathBuf::from("/dev/null/named-core")]);
        assert_eq!(
            mgba, gpsp,
            "the override stopped winning for one of the cores"
        );
    }
}
