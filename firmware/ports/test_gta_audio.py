from pathlib import Path
import subprocess,tempfile,os
with tempfile.TemporaryDirectory() as t:
 p=Path(t);good=p/'slot-alsoft.conf';bad=p/'game-alsoft.conf'
 good.write_text('[alsa]\ndevice=slot_ports\n')
 bad.write_text('[general]\ndrivers=alsa\n[alsa]\ndevice=nonexistent_bypass_device\n')
 (p/'alsa.conf').write_text('</usr/share/alsa/alsa.conf>\npcm.slot_ports { type file slave.pcm "null" file "'+str(p/'audio.raw')+'" format "raw" }\n')
 env=dict(os.environ,LD_PRELOAD=str(Path(os.environ.get('SLOT_GTA_AUDIO_HELPER','./slot-gta-audio.so')).resolve()),SLOT_GTA_ALSOFT_CONF=str(good),ALSA_CONFIG_PATH=str(p/'alsa.conf'),ALSOFT_DRIVERS='alsa',ALSOFT_LOGLEVEL='3')
 r=subprocess.run([os.environ.get('SLOT_GTA_AUDIO_TEST_BINARY','./test-gta-audio'),str(good),str(bad)],env=env,text=True,capture_output=True)
 Path(os.environ.get('SLOT_GTA_AUDIO_TEST_LOG','./gta-audio-tests.log')).write_text(r.stdout+r.stderr)
 assert r.returncode==0,r.stdout+r.stderr
 assert (p/'audio.raw').stat().st_size>0,'OpenAL did not write to Slot PCM'
 print('PASS real OpenAL output wrote', (p/'audio.raw').stat().st_size,'bytes to pinned Slot PCM')
