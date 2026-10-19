#!/bin/bash
set -eu
id="$1"
root="$2"
ports=/storage/roms/ports
export HOME=/storage
export XDG_DATA_HOME=/storage/.local/share
export SDL_VIDEODRIVER=kmsdrm
export SDL_AUDIODRIVER=alsa
export AUDIODEV="${SLOT_PORT_PCM:-${SLOT_ALSA_DEVICE:-plughw:0,0}}"
export ALSOFT_DRIVERS=alsa
export SDL_VIDEO_GL_DRIVER=libGLESv2.so.2
export SDL_VIDEO_EGL_DRIVER=libEGL.so.1
export SDL_GAMECONTROLLERCONFIG_FILE="$ports/PortMaster/gamecontrollerdb.txt"
export MALLOC_CHECK_=0
# Leave Menu and the physical volume rocker visible to the supervisor.
no_grab="$root/System/slot-no-grab.so"
[ -f "$no_grab" ] || no_grab=/usr/lib/slot-no-grab.so
if [ -f "$no_grab" ]; then
    export LD_PRELOAD="$no_grab${LD_PRELOAD:+:$LD_PRELOAD}"
fi
unset DISPLAY WAYLAND_DISPLAY
mkdir -p "$root/Config" "$HOME/.local/share"
# SDL uses AUDIODEV; OpenAL has its own ALSA device setting.
export ALSOFT_CONF="$root/Config/ports-alsoft.conf"
printf '[alsa]\ndevice = %s\n' "$AUDIODEV" > "$ALSOFT_CONF"
# Native SDL games use the same built-in GO-Super controller as the base OS.
export SDL_GAMECONTROLLERCONFIG='190000004b4800000011000000010000,GO-Super Gamepad,a:b1,b:b0,back:b12,dpdown:b9,dpleft:b10,dpright:b11,dpup:b8,guide:b16,leftshoulder:b4,leftstick:b14,lefttrigger:b6,leftx:a0,lefty:a1,rightshoulder:b5,rightstick:b15,righttrigger:b7,rightx:a2,righty:a3,start:b13,x:b2,y:b3,platform:Linux,'
echo "slot ports: starting $id $(date)"
case "$id" in
nfsu2)
    cd "$ports/nfsu2"
    export NFSU2_LANG=English NFSU2_DRAW_DIST=200 NFSU2_ASPECT=4:3
    export NFSU2_WIDESCREEN_DAT="$PWD/widescreen/NFSUnderground2.WidescreenFix.dat"
    export NFSU2_REGISTRY='NotFirstTime=1;g_MotionBlurEnable=0;g_LightStreaksEnable=0;g_DepthOfFieldEnable=0;g_OverBrightEnable=0;g_BleachByPassEnable=0;g_FSAALevel=0;g_RainEnable=0;g_RacingResolution=0;g_VSyncOn=0;g_CarDamageEnable=0;g_PerformanceLevel=1;g_TextureFiltering=1;g_CarEnvironmentMapEnable=0;g_CarEnvironmentMapUpdateData=0;g_CarShadowEnable=1;g_CarHeadlightEnable=1;g_CarLightingEnable=1;g_CrowdEnable=0;g_RoadReflectionEnable=0;g_FogEnable=1;g_HorizonFogEnable=1;g_LightGlowEnable=0;g_AnimatedTextureEnable=1;g_ParticleSystemEnable=0;g_WorldLodLevel=0;g_CarLodLevel=0;g_TintingEnable=0'
    exec ./nfsu2.aarch64 "$PWD/gamedata" --cap30 --lang=English >> "$PWD/log.txt" 2>&1
    ;;
gtasa)
    cd "$ports/gtasa"
    # OBBs use real filenames rather than symlinks: this card is exFAT.
    exec ./gtasa >> "$PWD/gtasa.log" 2>&1
    ;;
portmaster)
    cd "$ports/PortMaster"
    # Launch the GUI directly: its generic launcher assumes an EmulationStation
    # service and a separate Love splash runtime that this firmware does not use.
    exec /bin/bash -c 'source "$1/control.txt"; get_controls; export PYSDL2_DLL_PATH=/usr/lib; cd "$1"; exec /usr/bin/python3 ./pugwash' slot-portmaster "$ports/PortMaster"
    ;;
script:*)
    name="${id#script:}"
    case "$name" in *[!a-zA-Z0-9\ ._\(\)-]*|.*|*/*) exit 2;; esac
    case "$name" in *.sh) ;; *) exit 2;; esac
    cd "$ports"
    exec /bin/bash "$ports/$name"
    ;;
*) echo 'Unknown port id';exit 2;;
esac
