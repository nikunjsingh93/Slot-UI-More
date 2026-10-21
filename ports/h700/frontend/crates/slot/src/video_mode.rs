use std::path::Path;

use slot_gfx::{SRC_H, SRC_W, WHOLE_TEXTURE};
use slot_store::{ini, Platform};

pub const VIDEO_MODE_FILE: &str = "Config/video_mode.ini";

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum VideoMode {
    #[default]
    Actual,
    Stretch,
    HeightFit,
}

impl VideoMode {
    pub fn as_str(self) -> &'static str {
        match self {
            VideoMode::Actual => "actual",
            VideoMode::Stretch => "stretch",
            VideoMode::HeightFit => "height-fit",
        }
    }

    pub fn parse(s: &str) -> Option<VideoMode> {
        match s.trim().to_ascii_lowercase().as_str() {
            "actual" => Some(VideoMode::Actual),
            "stretch" => Some(VideoMode::Stretch),
            "height-fit" => Some(VideoMode::HeightFit),
            _ => None,
        }
    }

    pub fn next_l1(self) -> Self {
        match self {
            Self::Actual | Self::HeightFit => Self::Stretch,
            Self::Stretch => Self::HeightFit,
        }
    }
}

pub fn fills_height(platform: Platform, mode: VideoMode) -> bool {
    matches!(platform, Platform::Gb | Platform::Gbc) && mode != VideoMode::Actual
}

/// Width within the physical panel: RG SP is 3:2, RG35XX SP/G350 are 4:3.
pub fn width_fraction(platform: Platform, mode: VideoMode) -> f32 {
    let rgsp = std::env::var("SLOT_DEVICE").as_deref() == Ok("rgsp");
    let panel_aspect = if rgsp { 3.0 / 2.0 } else { 4.0 / 3.0 };
    if rgsp && matches!(platform, Platform::Ps1 | Platform::Snes | Platform::N64 | Platform::Dreamcast) {
        // Keep 4:3 consoles centered within the 3:2 panel. PSP uses full width.
        return (4.0 / 3.0) / panel_aspect;
    }
    if fills_height(platform, mode) && mode == VideoMode::HeightFit {
        let (w, h) = platform.picture();
        (w as f32 / h as f32 / panel_aspect).min(1.0)
    } else {
        1.0
    }
}

pub fn video_mode_for(root: &Path, stem: &str) -> VideoMode {
    ini::value(root, VIDEO_MODE_FILE, stem)
        .as_deref()
        .and_then(VideoMode::parse)
        .unwrap_or_default()
}

pub fn write_video_mode(root: &Path, stem: &str, mode: VideoMode) -> std::io::Result<()> {
    ini::write(root, VIDEO_MODE_FILE, stem, mode.as_str())
}

pub fn source_rect(platform: Platform, mode: VideoMode) -> [f32; 4] {
    if matches!(
        platform,
        Platform::Ps1 | Platform::Snes | Platform::N64 | Platform::Dreamcast | Platform::Psp
    ) {
        return WHOLE_TEXTURE;
    }
    let (w, h) = platform.picture();
    if mode == VideoMode::Actual || (w, h) == (SRC_W, SRC_H) {
        return WHOLE_TEXTURE;
    }
    let x = SRC_W.saturating_sub(w) / 2;
    let y = SRC_H.saturating_sub(h) / 2;
    [
        x as f32 / SRC_W as f32,
        y as f32 / SRC_H as f32,
        w as f32 / SRC_W as f32,
        h as f32 / SRC_H as f32,
    ]
}
