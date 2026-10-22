use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Platform {
    #[default]
    Gba,
    Gb,
    Gbc,
    Ps1,
    Snes,
    N64,
    Dreamcast,
    Psp,
    Ports,
    Nes,
    MegaDrive,
    MasterSystem,
    GameGear,
    PcEngine,
    Arcade,

}

impl Platform {
    pub const ALL: [Platform; 15] = [
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

    ];

    pub fn name(self) -> &'static str {
        match self {
            Platform::Gba => "Game Boy Advance",
            Platform::Gb => "Game Boy",
            Platform::Gbc => "Game Boy Color",
            Platform::Ps1 => "PlayStation",
            Platform::Snes => "Super Nintendo",
            Platform::N64 => "Nintendo 64",
            Platform::Dreamcast => "Dreamcast",
            Platform::Psp => "PSP",
            Platform::Ports => "Ports",
            Platform::Nes => "Nintendo Entertainment System",
            Platform::MegaDrive => "Mega Drive / Genesis",
            Platform::MasterSystem => "Master System",
            Platform::GameGear => "Game Gear",
            Platform::PcEngine => "PC Engine / TurboGrafx-16",
            Platform::Arcade => "Arcade (FinalBurn Neo)",
        }
    }

    pub fn dir_name(self) -> &'static str {
        match self {
            Platform::Gba => "GBA",
            Platform::Gb => "GB",
            Platform::Gbc => "GBC",
            Platform::Ps1 => "PS1",
            Platform::Snes => "SNES",
            Platform::N64 => "N64",
            Platform::Dreamcast => "DC",
            Platform::Psp => "PSP",
            Platform::Ports => "PORTS",
            Platform::Nes => "NES",
            Platform::MegaDrive => "MD",
            Platform::MasterSystem => "SMS",
            Platform::GameGear => "GG",
            Platform::PcEngine => "PCE",
            Platform::Arcade => "ARCADE",
        }
    }

    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Platform::Ports => &["port"],
            Platform::Nes => &["nes", "unf", "unif"],
            Platform::MegaDrive => &["md", "gen", "bin", "smd"],
            Platform::MasterSystem => &["sms"],
            Platform::GameGear => &["gg"],
            Platform::PcEngine => &["pce"],
            Platform::Arcade => &["zip"],
            Platform::Gba => &["gba"],
            Platform::Gb | Platform::Gbc => &["gb", "gbc"],
            Platform::Ps1 => &["chd", "cue", "m3u", "pbp", "exe"],
            Platform::Snes => &["sfc", "smc"],
            Platform::N64 => &["z64", "n64", "v64"],
            Platform::Dreamcast => &["chd", "gdi", "cdi", "cue", "m3u", "elf"],
            Platform::Psp => &["iso", "cso", "pbp", "chd", "elf"],
        }
    }

    pub fn picture(self) -> (u32, u32) {
        match self {
            Platform::Gba => (240, 160),
            Platform::Gb | Platform::Gbc => (160, 144),
            Platform::Ps1 | Platform::Snes | Platform::N64 | Platform::Dreamcast => (640, 480),
            Platform::Psp => (480, 272),
            Platform::Ports => (640, 480),
            Platform::GameGear => (160, 144),
            Platform::Nes | Platform::MegaDrive | Platform::MasterSystem | Platform::PcEngine | Platform::Arcade => (640, 480),
        }
    }

    pub fn accepts(self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                self.extensions()
                    .iter()
                    .any(|k| ext.eq_ignore_ascii_case(k))
            })
    }
}
