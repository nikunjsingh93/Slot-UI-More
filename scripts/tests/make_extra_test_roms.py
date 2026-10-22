"""Generate tiny original idle/test ROMs; no commercial code or assets.

These test the loader, frame/audio bridge and state roundtrip, not game accuracy.
"""
import argparse,struct,zipfile
from pathlib import Path

def main():
    p=argparse.ArgumentParser();p.add_argument('out',type=Path);out=p.parse_args().out;out.mkdir(parents=True,exist_ok=True)
    prg=bytearray([0xea])*16384
    # Disable interrupts, set background palette, enable background, loop.
    code=bytes.fromhex('78 d8 a2 ff 9a a9 00 8d 00 20 8d 01 20 2c 02 20 a9 3f 8d 06 20 a9 00 8d 06 20 a9 21 8d 07 20 a9 08 8d 01 20 4c 24 80')
    prg[:len(code)]=code
    prg[-6:]=struct.pack('<HHH',0x8024,0x8000,0x8024)
    (out/'test.nes').write_bytes(b'NES\x1a'+bytes([1,1])+bytes(10)+prg+bytes(8192))
    md=bytearray(32768);md[:8]=struct.pack('>II',0xff0000,0x200)
    md[0x100:0x110]=b'SEGA GENESIS    '
    md[0x120:0x150]=b'SLOT ORIGINAL BRIDGE TEST'.ljust(48)
    md[0x180:0x18e]=b'GM 00000000-00'
    md[0x1a0:0x1b0]=struct.pack('>IIII',0,32767,0xff0000,0xffffff)
    md[0x1f0:0x1f3]=b'JUE'
    md[0x200:0x208]=bytes.fromhex('46 fc 27 00 60 fe 4e 71')
    (out/'test.md').write_bytes(md)
    sms=bytearray(32768);sms[:3]=bytes.fromhex('c3 00 01');sms[0x100:0x105]=bytes.fromhex('f3 00 c3 01 01');sms[0x7ff0:0x7ff8]=b'TMR SEGA'
    sms[0x7fff]=0x40;(out/'test.sms').write_bytes(sms)
    sms[0x7fff]=0x60;(out/'test.gg').write_bytes(sms)
    pce=bytearray([0xea])*8192;pce[:4]=bytes.fromhex('78 4c 01 e0');pce[-2:]=struct.pack('<H',0xe000);(out/'test.pce').write_bytes(pce)
    with zipfile.ZipFile(out/'missing-test.zip','w') as z:z.writestr('README.txt','Original invalid-archive rejection fixture; no ROMs.')

if __name__=='__main__':main()
