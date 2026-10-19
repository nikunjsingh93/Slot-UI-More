"""Prepare the G350 manager bundle from the pinned official release.

Usage: prepare_manager.py OFFICIAL_PortMaster.zip OUTPUT.zip [PUBLIC_SOURCE_CACHE.json]
The optional cache is upstream catalogue metadata, never user settings or games.
"""
from pathlib import Path
import hashlib,json,sys,zipfile
source,output=map(Path,sys.argv[1:3])
expected='cde44901ed0dba0be7d4ab8f18149545dc167bad12a99fba76304cf1986a3a07'
assert hashlib.sha256(source.read_bytes()).hexdigest()==expected,'Unexpected official manager ZIP'
assert source.resolve()!=output.resolve()
with zipfile.ZipFile(source) as z,zipfile.ZipFile(output,'w',zipfile.ZIP_DEFLATED,compresslevel=5) as out:
    for entry in z.infolist():out.writestr(entry,z.read(entry))
    if len(sys.argv)>3:
        cache=json.loads(Path(sys.argv[3]).read_text())
        assert cache['api']=='PortMasterV3' and cache['version']==2
        assert cache['url']=='https://github.com/PortsMaster/PortMaster-New/releases/latest/download/ports.json'
        # Retain only the public source schema, excluding accidental user fields.
        clean={k:cache[k] for k in ['prefix','api','name','url','last_checked','version','data']}
        out.writestr('PortMaster/config/020_portmaster.source.json',json.dumps(clean,indent=2)+'\n')
print('PASS pinned upstream manager and optional public catalogue packaged')
