"""Replace only a G350 kernel's embedded early-boot image/banner.

Usage: python patch_boot_splash.py ORIGINAL_KERNEL PALETTE_PNG OUTPUT_KERNEL
The gzip stream stays within its original allocation; kernel code and offsets
are unchanged. No device-tree, mounting, filesystem or boot options are edited.
"""
from pathlib import Path
import sys,gzip,zlib,hashlib,json,struct

def records(data):
    pos=0;out=[]
    while data[pos:pos+6]==b'070701':
        header=data[pos:pos+110]
        fields=[int(header[6+i*8:14+i*8],16) for i in range(13)]
        size,namesize=fields[6],fields[11]
        name=data[pos+110:pos+110+namesize-1].decode()
        start=(pos+110+namesize+3)&~3
        out.append((name,header,data[pos+110:pos+110+namesize],data[start:start+size]))
        pos=(start+size+3)&~3
        if name=='TRAILER!!!':
            assert not any(data[pos:]),'Unexpected data after the startup archive'
            return out
    raise ValueError('Incomplete initramfs archive')

def patch(kernel,logo):
    assert kernel[56:60]==b'ARM\x64','Expected uncompressed ARM64 Image'
    assert logo[:8]==b'\x89PNG\r\n\x1a\n' and struct.unpack('>II',logo[16:24])==(640,480),'Expected native 640x480 PNG'
    candidates=[];pos=0
    while (pos:=kernel.find(b'\x1f\x8b\x08',pos))>=0:
        start=pos;pos+=3
        try:
            dec=zlib.decompressobj(31);data=dec.decompress(kernel[start:])
            if not dec.eof or not data.startswith(b'070701'):continue
            entries=records(data)
            names={n for n,_,_,_ in entries}
            if {'init','splash/splash-640.png'}<=names:
                candidates.append((start,len(kernel)-start-len(dec.unused_data),entries))
        except (ValueError,zlib.error,UnicodeError):continue
    assert len(candidates)==1,'Expected one embedded G350 initramfs'
    start,budget,entries=candidates[0];rebuilt=bytearray();changed=[]
    for name,header,encoded,data in entries:
        original=data
        if name=='splash/splash-640.png':data=logo
        if name=='init':
            lines=data.splitlines(keepends=True)
            banner=[line for line in lines if b'echo -en' in line and b'${COREELEC_DEVICE}' in line]
            assert len(banner)==1,'Expected original device-imager console banner'
            data=b''.join(line for line in lines if line not in banner)
        if data!=original:changed.append(name)
        header=header[:54]+f'{len(data):08X}'.encode()+header[62:]
        rebuilt.extend(header+encoded);rebuilt.extend(b'\0'*(-len(rebuilt)%4))
        rebuilt.extend(data);rebuilt.extend(b'\0'*(-len(rebuilt)%4))
    rebuilt.extend(b'\0'*(-len(rebuilt)%512))
    packed=gzip.compress(bytes(rebuilt),compresslevel=9,mtime=0)
    assert len(packed)<=budget,f'New startup archive exceeds allocation: {len(packed)} > {budget}'
    result=kernel[:start]+packed+b'\0'*(budget-len(packed))+kernel[start+budget:]
    assert len(result)==len(kernel)
    assert result[:start]==kernel[:start] and result[start+budget:]==kernel[start+budget:]
    dec=zlib.decompressobj(31);actual=records(dec.decompress(result[start:]))
    assert len(actual)==len(entries)
    expected_changed={'init','splash/splash-640.png'}
    assert set(changed)==expected_changed
    for before,after in zip(entries,actual):
        assert before[:1]==after[:1] and before[2]==after[2]
        assert before[1][:54]+before[1][62:]==after[1][:54]+after[1][62:]
        if before[0] not in expected_changed:assert before[3]==after[3]
    return result,{'gzip_offset':start,'original_gzip_size':budget,'new_gzip_size':len(packed),'changed':changed,'kernel_code_unchanged':True}

if __name__=='__main__':
    src,logo,dest=map(Path,sys.argv[1:]);original=src.read_bytes()
    result,receipt=patch(original,logo.read_bytes());dest.write_bytes(result)
    receipt.update(original_sha256=hashlib.sha256(original).hexdigest(),kernel_sha256=hashlib.sha256(result).hexdigest())
    dest.with_suffix(dest.suffix+'.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt))
