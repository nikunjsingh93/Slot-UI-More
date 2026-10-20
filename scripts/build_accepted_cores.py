"""Build the accepted G350 Dreamcast/N64 cores from verified public sources.

Run inside the ARM64 builder. Cache contains only upstream source archives;
outputs never include games, BIOS or personal save states.
"""
from pathlib import Path
import argparse
import hashlib
import json
import shutil
import subprocess
import urllib.request

p = argparse.ArgumentParser()
p.add_argument("repo", type=Path)
p.add_argument("build", type=Path)
p.add_argument("--cache", type=Path)
args = p.parse_args()
repo, build = args.repo.resolve(), args.build.resolve()
recipes = repo / "experiments/g350-performance-20261019"
cache = args.cache or build / "downloads"
cache.mkdir(parents=True, exist_ok=True)
sources = build / "accepted-core-source"
sources.mkdir(parents=True, exist_ok=True)
outputs = build / "accepted-cores"
outputs.mkdir(parents=True, exist_ok=True)


def run(command, cwd):
    subprocess.run(command, cwd=cwd, check=True)


def source(name, metadata, patches):
    info = json.loads((recipes / metadata).read_text())
    archive = cache / info["archive"]
    if not archive.exists():
        partial = archive.with_suffix(archive.suffix + ".partial")
        with urllib.request.urlopen(info["url"], timeout=60) as src, partial.open("wb") as dst:
            shutil.copyfileobj(src, dst)
        partial.replace(archive)
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    if digest != info["sha256"]:
        raise RuntimeError(f"Source checksum mismatch: {archive}")
    destination = sources / name
    stamp = destination / ".slot-accepted-source.json"
    recipe = {"source": info, "patches": {
        patch: hashlib.sha256((recipes / patch).read_bytes()).hexdigest()
        for patch in patches
    }}
    if destination.exists():
        if not stamp.exists() or json.loads(stamp.read_text()) != recipe:
            raise RuntimeError(f"Use a new build directory; source recipe changed: {destination}")
        return destination
    destination.mkdir()
    # Archives are pinned by the verified digest before extraction.
    run(["tar", "-xzf", str(archive), "--strip-components=1", "-C", str(destination)], build)
    for patch in patches:
        run(["patch", "-p1", "-i", str(recipes / patch)], destination)
    if name == "flycast":
        makefile = destination / "Makefile"
        makefile.write_text(makefile.read_text().replace("cortex-a53", "cortex-a35"))
    stamp.write_text(json.dumps(recipe, indent=2) + "\n")
    return destination


flycast = source("flycast", "flycast2021-source.json", [
    "flycast2021-postprocess.patch",
    "flycast2021-aarch64-abi.patch",
    "flycast2021-slot-callback-bridge.patch",
])
parallel = source("parallel", "parallel-current-source.json", [])
run(["make", "-j4", "platform=arm64_cortex_a53_gles2", "FORCE_GLES=1",
     "HAVE_VULKAN=0", "HAVE_OPENMP=0", "HAVE_LTCG=0", "GIT_VERSION=603814c"], flycast)
settings = json.loads((recipes / "n64-lto-build.json").read_text())
run(["make", "-j4", *settings["make_options"].split(),
     "CPUFLAGS=" + settings["cpu_flags"]], parallel)
for directory, name in [(flycast, "flycast_libretro.so"), (parallel, "parallel_n64_libretro.so")]:
    output = outputs / name
    shutil.copyfile(directory / name, output)
    run(["strip", "--strip-unneeded", str(output)], build)
    print(f"Built {name}: {hashlib.sha256(output.read_bytes()).hexdigest()}", flush=True)

# Keep license notices, pinned source references and applied patches with binaries.
for directory, name in [(flycast, "flycast"), (parallel, "parallel_n64")]:
    notices = outputs / "notices" / name
    notices.mkdir(parents=True, exist_ok=True)
    for filename in ["LICENSE", "LICENSE.TXT", "LICENSE.txt", "COPYING", "COPYING.txt"]:
        if (directory / filename).is_file():
            shutil.copyfile(directory / filename, notices / filename)
    shutil.copyfile(directory / ".slot-accepted-source.json", notices / "sources.json")
    if name == "flycast":
        for patch in json.loads((directory / ".slot-accepted-source.json").read_text())["patches"]:
            shutil.copyfile(recipes / patch, notices / patch)
shutil.copyfile(recipes / "n64-lto-build.json", outputs / "notices/parallel_n64/build.json")
