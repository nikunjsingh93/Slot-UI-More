#!/bin/sh
# User supplies their matching private runtime and original PC game data.
GAMEDIR="${SDCARD_PATH}/ports/.ports/nfsu2"
cd "$GAMEDIR" || exit 1
unset SDL_VIDEODRIVER DISPLAY WAYLAND_DISPLAY
export SDL_AUDIODRIVER=alsa
export SDL_VIDEO_GL_DRIVER=libGLESv2.so.2 SDL_VIDEO_EGL_DRIVER=libEGL.so.1
export NFSU2_LANG=English NFSU2_RENDER=320x240 NFSU2_ASPECT=4:3
export NFSU2_DRAW_DIST=200 NFSU2_FORCE_PAD_MAPPING=1
export NFSU2_REGISTRY='NotFirstTime=1;g_MotionBlurEnable=0;g_LightStreaksEnable=0;g_DepthOfFieldEnable=0;g_OverBrightEnable=0;g_BleachByPassEnable=0;g_FSAALevel=0;g_RainEnable=0;g_RacingResolution=0;g_VSyncOn=0;g_CarDamageEnable=0;g_PerformanceLevel=1;g_TextureFiltering=1;g_CarEnvironmentMapEnable=0;g_CarEnvironmentMapUpdateData=0;g_CarShadowEnable=1;g_CarHeadlightEnable=1;g_CarLightingEnable=1;g_CrowdEnable=0;g_RoadReflectionEnable=0;g_FogEnable=1;g_HorizonFogEnable=1;g_LightGlowEnable=0;g_AnimatedTextureEnable=1;g_ParticleSystemEnable=0;g_WorldLodLevel=0;g_CarLodLevel=0;g_TintingEnable=0'
# Matched GO-Super virtual layout: native menu/driving controls, including R3.
# Menu+X synthesizes R3 for the name-entry keyboard; Menu-alone still exits.
exec ./nfsu2.aarch64 "$PWD/gamedata" --cap30 --lang=English >> log.txt 2>&1
