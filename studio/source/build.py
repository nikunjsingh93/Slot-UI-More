"""Rebuild the standalone studio from the bundled catalog and sources."""
from pathlib import Path
import argparse, json, hashlib

w=Path(__file__).resolve().parent
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output',type=Path,default=w.parent/'Slot-Auto-Artwork-Studio-v4.html')
args=parser.parse_args()
catalog=json.loads((w/'catalog.json').read_text(encoding='utf-8'))
html=(w/'template.html').read_text(encoding='utf-8')
replacements={
    '__CATALOG_DATE__':catalog['date'],
    '__CATALOG__':json.dumps(catalog,ensure_ascii=False,separators=(',',':')).replace('<','\\u003c'),
    '__JSZIP__':(w/'jszip.min.js').read_text(encoding='utf-8'),
    '__APPLICATION__':(w/'app.js').read_text(encoding='utf-8'),
}
for marker,value in replacements.items():
    assert marker in html, f'Missing template marker: {marker}'
    html=html.replace(marker,value)
assert all(marker not in html for marker in replacements)
args.output.parent.mkdir(parents=True,exist_ok=True)
args.output.write_text(html,encoding='utf-8')
print(args.output, hashlib.sha256(args.output.read_bytes()).hexdigest())
