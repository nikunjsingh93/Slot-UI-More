"""Install the bundled manager once; never replace an existing installation."""
from pathlib import Path
import os, shutil, sys, zipfile

root, archive = map(Path, sys.argv[1:])
destination = root.parent / 'ports' / 'PortMaster'
if destination.joinpath('pugwash').is_file() or not archive.is_file():
    sys.exit(0)
destination.parent.mkdir(parents=True, exist_ok=True)
stage = destination.parent / '.slot-portmaster-install'
stage.mkdir(exist_ok=True)
with zipfile.ZipFile(archive) as z:
    for entry in z.infolist():
        parts = Path(entry.filename).parts
        assert parts and parts[0] == 'PortMaster' and '..' not in parts
        target = stage.joinpath(*parts)
        assert target.resolve().is_relative_to(stage.resolve())
    assert z.testzip() is None
    z.extractall(stage)
assert (stage / 'PortMaster/pugwash').is_file()
if destination.exists():
    # A partial/manual install may hold user settings: do not remove it.
    raise RuntimeError('PortMaster directory already exists but is incomplete')
os.rename(stage / 'PortMaster', destination)
stage.rmdir()
print('slot: bundled PortMaster installed; future boots preserve it')
