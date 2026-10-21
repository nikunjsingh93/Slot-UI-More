#!/usr/bin/env python3
"""Slot H700 direct Mupen64Plus/Rice host. MIT; emulator licences separate."""
import ctypes as C
import os,sys,signal,threading,time,configparser
from pathlib import Path
BASE=Path(__file__).resolve().parent
PLUGIN_DIR=BASE/'plugins'
DATA_DIR=BASE
CORE_PATH=str(BASE/'libmupen64plus.so.2')
DEBUG=C.CFUNCTYPE(None,C.c_void_p,C.c_int,C.c_char_p)
STATE=C.CFUNCTYPE(None,C.c_void_p,C.c_int,C.c_int)
def log(s): print('slot: native-rice: '+s,flush=True)
def consume_request(root):
    request=root/'Config/n64-request'
    data=request.read_bytes();request.unlink()
    if len(data)>4096 or b'\0' in data: raise ValueError('invalid request')
    rom=Path(os.fsdecode(data)).resolve(strict=True)
    if not rom.is_relative_to((root/'Games/N64').resolve()) or rom.suffix.lower() not in ('.z64','.n64','.v64'): raise ValueError('invalid ROM path')
    if not 4096<=rom.stat().st_size<=128*1024*1024: raise ValueError('invalid ROM size')
    return rom
def make_config(root):
    directory=root/'Config/N64-Rice';directory.mkdir(exist_ok=True)
    cfg=configparser.ConfigParser(interpolation=None,strict=False);cfg.optionxform=str
    cfg.read(BASE/'muos-reference.cfg')
    # Slot INI files store bare key=value lines rather than section headers.
    path=root/'Config/screen-size.ini'
    values=dict(tuple(part.strip() for part in line.split('=',1)) for line in path.read_text().splitlines() if '=' in line and not line.lstrip().startswith(('#',';'))) if path.exists() else {}
    wide=values.get('N64')=='full'
    panel=720 if os.environ.get('SLOT_DEVICE')=='rgsp' else 640
    settings={
      'Core':{'R4300Emulator':2,'NoCompiledJump':False,'OnScreenDisplay':False,'SharedDataPath':str(BASE),'SaveFilenameFormat':1,'SaveSRAMPath':str(root/'Saves/N64/standalone-rice'),'SaveStatePath':str(root/'States/N64/standalone-rice'),'ScreenshotPath':str(root/'Screenshots/N64')},
      'Video-General':{'Fullscreen':True,'ScreenWidth':panel if wide else 640,'ScreenHeight':480,'VerticalSync':False},
      'Video-Rice':{'ResolutionWidth':panel,'FrameBufferSetting':1,'FrameBufferWriteBackControl':1,'RenderToTexture':0,'ScreenUpdateSetting':4,'FastTextureLoading':True,'InN64Resolution':False,'LoadHiResTextures':False,'TextureEnhancement':0,'MultiSampling':0,'AnisotropicFiltering':0,'SkipFrame':False},
      'Audio-SDL':{'PRIMARY_BUFFER_SIZE':8192,'PRIMARY_BUFFER_TARGET':4096,'SECONDARY_BUFFER_SIZE':2048,'RESAMPLE':'trivial','AUDIO_SYNC':True,'VOLUME_CONTROL_TYPE':1,'VOLUME_DEFAULT':100},
    }
    for folder in ['Saves/N64/standalone-rice','States/N64/standalone-rice','Screenshots/N64']:(root/folder).mkdir(parents=True,exist_ok=True)
    # Disable all firmware-specific shortcuts before installing Slot chords.
    for key in list(cfg['CoreEvents']):
        if key.startswith('Joy Mapping'): cfg['CoreEvents'][key]='""'
    cfg['CoreEvents']['Joy Mapping Save State']='"J0B16/B2"'
    cfg['CoreEvents']['Joy Mapping Load State']='"J0B16/B3"'
    for section,items in settings.items():
        if not cfg.has_section(section):cfg.add_section(section)
        for key,value in items.items():cfg[section][key]=str(value) if isinstance(value,(int,bool)) else '"'+value+'"'
    controls={'version':'2.000000','mode':'0','device':'0','name':'"GO-Super Gamepad"','plugged':'True','plugin':'2','mouse':'False','AnalogDeadzone':'"4096,4096"','AnalogPeak':'"32767,32767"','Start':'"button(13)"','A Button':'"button(1)"','B Button':'"button(0)"','Z Trig':'"button(6)"','L Trig':'"button(4)"','R Trig':'"button(5)"','C Button L':'"button(3)"','C Button D':'"button(12)"','C Button R':'"button(7)"','C Button U':'"button(2)"','DPad U':'"hat(0 Up)"','DPad D':'"hat(0 Down)"','DPad L':'"hat(0 Left)"','DPad R':'"hat(0 Right)"','X Axis':'"axis(0-,0+)"','Y Axis':'"axis(1-,1+)"'}
    for i in range(1,5):
        section='Input-SDL-Control'+str(i)
        if not cfg.has_section(section):cfg.add_section(section)
        cfg[section].update(controls)
        if i!=1:cfg[section]['plugged']='False';cfg[section]['device']='-1'
    p=directory/'mupen64plus.cfg'
    with p.open('w') as f:cfg.write(f);f.flush();os.fsync(f.fileno())
    return directory
class Core:
    def __init__(self, root, directory):
        self.root = root
        self.lib = C.CDLL(CORE_PATH, mode=C.RTLD_GLOBAL)
        self.running = threading.Event()
        self.saved = threading.Event()
        self.plugins = []
        self.rom = None
        self.opened = False
        self.debug = DEBUG(self.debug_message)
        self.state = STATE(self.state_changed)
        self.lib.CoreStartup.argtypes = [C.c_int, C.c_char_p, C.c_char_p,
                                        C.c_void_p, DEBUG, C.c_void_p, STATE]
        self.lib.CoreAttachPlugin.argtypes = [C.c_int, C.c_void_p]
        self.lib.CoreDetachPlugin.argtypes = [C.c_int]
        self.lib.CoreDoCommand.argtypes = [C.c_int, C.c_int, C.c_void_p]
        self.check(self.lib.CoreStartup(0x020001, os.fsencode(directory),
                                       os.fsencode(DATA_DIR), None, self.debug,
                                       None, self.state), 'CoreStartup')

    def debug_message(self, context, level, message):
        if level <= 4:
            log(message.decode('utf-8', 'replace'))

    def state_changed(self, context, parameter, value):
        if parameter == 11:
            self.saved.set()
        if parameter == 1 and value == 2:
            self.running.set()

    @staticmethod
    def check(result, action):
        if result:
            raise RuntimeError(f'{action} failed (Mupen64Plus error {result})')

    def open(self, path):
        self.rom = C.create_string_buffer(path.read_bytes())
        self.check(self.lib.CoreDoCommand(1, len(self.rom) - 1, self.rom), 'ROM_OPEN')
        self.opened = True
        settings = C.create_string_buffer(512)
        if self.lib.CoreDoCommand(4, len(settings), settings) == 0:
            migrate_save(self.root, path, settings.raw)
        for kind, name in [(2, 'video-rice'), (3, 'audio-sdl'),
                           (4, 'input-sdl'), (1, 'rsp-hle')]:
            plugin = C.CDLL(str(PLUGIN_DIR / ('mupen64plus-' + name + '.so')))
            plugin.PluginStartup.argtypes = [C.c_void_p, C.c_void_p, DEBUG]
            self.check(plugin.PluginStartup(self.lib._handle, None, self.debug),
                       name + ' PluginStartup')
            self.plugins.append((kind, plugin))
            self.check(self.lib.CoreAttachPlugin(kind, plugin._handle), name + ' attach')

    def execute(self):
        self.check(self.lib.CoreDoCommand(5, 0, None), 'EXECUTE')

    def stop(self):
        self.lib.CoreDoCommand(6, 0, None)

    def volume(self, value):
        gain = C.c_int(value)
        self.lib.CoreDoCommand(17, 7, C.byref(gain))

    def close(self):
        for kind, plugin in reversed(self.plugins):
            self.lib.CoreDetachPlugin(kind)
            plugin.PluginShutdown()
        if self.opened:
            self.lib.CoreDoCommand(2, 0, None)
        self.lib.CoreShutdown()


def migrate_save(root, rom, settings):
    """Copy known Mupen64Plus-Next packed SRAM once; never overwrite native saves.

    Layout verified against pinned libretro_memory.h: EEPROM, four mempaks,
    SRAM, FlashRAM. Native Mupen64Plus uses the same raw storage byte order.
    Savestates are deliberately separate; their formats are incompatible.
    """
    source = root / 'Saves/N64' / (rom.stem + '.sav')
    if not source.is_file() or source.stat().st_size != 296960:
        return
    goodname = settings[:256].split(b'\0', 1)[0]
    md5 = settings[256:289].split(b'\0', 1)[0]
    if len(md5) != 32 or any(c not in b'0123456789abcdefABCDEF' for c in md5):
        return
    if b'(unknown rom)' in goodname:
        # Unknown ROM naming depends on the normalized header. Leave its legacy
        # save intact rather than guess a destination.
        log('legacy save retained; unknown ROM has no safe native name')
        return
    basename = os.fsdecode(goodname[:32] + b'-' + md5[:8])
    if any(c in basename for c in ('/', '\\', '\0')):
        return
    destination = root / 'Saves/N64/standalone-rice'
    destination.mkdir(parents=True, exist_ok=True)
    data = source.read_bytes()
    parts = {0: ('.eep', 0, 512), 1: ('.eep', 0, 2048),
             2: ('.sra', 133120, 165888), 3: ('.fla', 165888, 296960)}
    outputs = [('.mpk', 2048, 133120)]
    if settings[289] in parts:
        outputs.append(parts[settings[289]])
    for extension, begin, end in outputs:
        target = destination / (basename + extension)
        if target.exists():
            continue
        temporary = target.with_suffix(extension + '.tmp')
        with temporary.open('wb') as f:
            f.write(data[begin:end]); f.flush(); os.fsync(f.fileno())
        temporary.replace(target)
        log('copied legacy in-game save to ' + target.name + '; original kept')



def main():
    root=Path(os.environ['SLOT_ROOT']).resolve();rom=consume_request(root)
    signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
    core=Core(root,make_config(root));core.open(rom)
    resume=root/'States/N64/standalone-rice'/(rom.stem+'.resume.st0')
    # A callback runs on the emulation thread; schedule state loading there.
    resume_bytes=os.fsencode(resume)
    @C.CFUNCTYPE(None,C.c_uint)
    def frame(index):
        if index==5 and resume.exists():core.lib.CoreDoCommand(10,0,C.c_char_p(resume_bytes))
    core.check(core.lib.CoreDoCommand(15,0,C.cast(frame,C.c_void_p)),'frame callback')
    def quit_waiter():
        signal.sigwait({signal.SIGTERM,signal.SIGINT})
        if core.running.wait(1):
            core.saved.clear();core.lib.CoreDoCommand(11,1,C.c_char_p(resume_bytes));core.saved.wait(1)
        core.stop()
    threading.Thread(target=quit_waiter,daemon=True).start()
    log('direct Rice GLES / HLE / ARM64 dynarec; hold Menu to quit; Menu+X save, Menu+Y load')
    try:core.execute()
    finally:core.close()
if __name__=='__main__':
    try:main()
    except Exception as e:log(str(e));sys.exit(1)
