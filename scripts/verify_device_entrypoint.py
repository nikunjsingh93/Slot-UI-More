"""Run the actual shipping executable on a machine with no display devices.

Expected failure must come from SDL KMS/DRM, not desktop winit. This checks
the compiled entrypoint, which the headless core integration probes bypass.
"""
from pathlib import Path
import os, subprocess, sys

binary = Path(sys.argv[1])
env = os.environ.copy()
for key in ['DISPLAY', 'WAYLAND_DISPLAY', 'WAYLAND_SOCKET']:
    env.pop(key, None)
env['SLOT_DEVICE'] = 'g350'
result = subprocess.run([str(binary)], env=env, capture_output=True,
                        text=True, timeout=15)
output = result.stdout + result.stderr
assert result.returncode == 1, (result.returncode, output)
assert 'SDL KMS/DRM init:' in output, output
assert 'KMSDRM not available' in output, output
assert 'winit' not in output and 'DISPLAY is set' not in output, output
print(output.strip())
print('PASS: shipping binary selects handheld KMS/DRM without desktop display variables')
