"""Patch an extracted G350 SYSTEM; never changes its boot files or cores.
Usage: patch_system.py EXTRACTED_ROOT FRONTEND_BINARY SUPERVISOR_BINARY INPUT_SHARING_SO
Repack the result with mksquashfs after this finishes.
"""
from pathlib import Path
import sys,shutil
root,frontend,supervisor,input_sharing=map(Path,sys.argv[1:])
here=Path(__file__).resolve().parent
assert (root/'usr/bin/slot-g350').is_file(), 'Expected extracted G350 SYSTEM'
for src,name in [(frontend,'slot-g350'),(supervisor,'slot-portwatch'),
                 (here.parent/'slot-start','slot-start'),
                 (here/'slot-port-runner.sh','slot-port-runner.sh'),
                 (here/'slot-refresh-ports.py','slot-refresh-ports.py'),
                 (here/'slot-provision-portmaster.py','slot-provision-portmaster.py'),
                 (here/'show_splash.sh','show_splash.sh')]:
    target=root/'usr/bin'/name;shutil.copyfile(src,target);target.chmod(0o755)
shutil.copyfile(input_sharing,root/'usr/lib/slot-no-grab.so')
(root/'usr/lib/slot-no-grab.so').chmod(0o755)
# The stock tmpfiles rule creates 117 unused system folders on every boot.
# Replace only its directory-creation list; never remove existing user data.
shutil.copyfile(here/'slot-directories.conf',root/'usr/lib/tmpfiles.d/amberelec-dirs.conf')
(root/'usr/lib/tmpfiles.d/amberelec-dirs.conf').chmod(0o644)
(root/'usr/share/slot/VERSION').write_text('0.1.14-complete-ports\n')
logo=here/'visuals/slot-splash.png'
sequence=root/'usr/share/p4_splash'
assert sequence.resolve().is_relative_to(root.resolve())
for image in sequence.glob('p4logo.*.jpg'):image.unlink()
shutil.copyfile(logo,sequence/'slot-logo.png')
shutil.copyfile(logo,root/'usr/config/splash/splash-640.png')
shutil.copyfile(logo,root/'usr/share/slot/splash.png')
alias=root/'PortMaster'
if not alias.is_symlink():alias.symlink_to('/storage/roms/ports/PortMaster')
