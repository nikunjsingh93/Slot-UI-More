"""Extend the embedded catalogue from GitHub tree JSON and LaunchBox metadata.

python add_gameboy_catalog.py --trees PATH --launchbox Metadata.zip
The tree directory must contain GB-tree.json, GBC-tree.json and GBA-tree.json.
Only image identifiers and game names are embedded; artwork is fetched on demand.
"""
import argparse, collections, json, zipfile
from pathlib import Path
import xml.etree.ElementTree as ET

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--trees',type=Path,required=True)
    p.add_argument('--launchbox',type=Path,required=True)
    p.add_argument('--systems',choices=['gameboy','extra'],default='gameboy')
    p.add_argument('--fbneo-dat',type=Path,help='Exact bundled Arcade-only FBNeo XML DAT for --systems extra')
    args=p.parse_args(); here=Path(__file__).resolve().parent
    catalog=json.loads((here/'catalog.json').read_text(encoding='utf-8'))
    platforms={'Nintendo Game Boy':'GB','Nintendo Game Boy Color':'GBC','Nintendo Game Boy Advance':'GBA'}
    repos={'GB':'Nintendo_-_Game_Boy','GBC':'Nintendo_-_Game_Boy_Color','GBA':'Nintendo_-_Game_Boy_Advance'}
    if args.systems=='extra':
        if not args.fbneo_dat:p.error('--systems extra requires --fbneo-dat')
        platforms={'Nintendo Entertainment System':'NES','Sega Genesis':'MD','Sega Master System':'SMS','Sega Game Gear':'GG','NEC TurboGrafx-16':'PCE','Arcade':'ARCADE'}
        repos={'NES':'Nintendo_-_Nintendo_Entertainment_System','MD':'Sega_-_Mega_Drive_-_Genesis','SMS':'Sega_-_Master_System_-_Mark_III','GG':'Sega_-_Game_Gear','PCE':'NEC_-_PC_Engine_-_TurboGrafx_16','ARCADE':'FBNeo_-_Arcade_Games'}
    selected={}; images=collections.defaultdict(list)
    with zipfile.ZipFile(args.launchbox) as z,z.open('Metadata.xml') as f:
        context=ET.iterparse(f,events=('start','end'));_,root=next(context)
        for event,e in context:
            if event!='end':continue
            if e.tag=='Game':
                platform=platforms.get(e.findtext('Platform'))
                if platform:selected[e.findtext('DatabaseID')]=(platform,e.findtext('Name'))
                root.clear()
            elif e.tag=='GameImage':
                key=e.findtext('DatabaseID')
                if key in selected and e.findtext('Type') in ['Cart - Front','Clear Logo','Box - Front']:
                    images[key].append([e.findtext('FileName'),e.findtext('Region') or '',e.findtext('Type')])
                root.clear()
    for platform,repo in repos.items():
        tree=json.loads((args.trees/(platform+'-tree.json')).read_text(encoding='utf-8'))
        assert not tree.get('truncated'),'Incomplete GitHub tree'
        titles={}
        for e in tree['tree']:
            folder,_,filename=e['path'].partition('/')
            bit={'Named_Logos':1,'Named_Boxarts':2,'Named_Titles':4}.get(folder,0)
            if e['type']=='blob' and bit and filename.endswith('.png') and '/' not in filename:
                title=filename[:-4];titles[title]=titles.get(title,0)|bit
        physical=[];supplemental=[]
        for key,(system,title) in selected.items():
            if system!=platform:continue
            scans=[[a,b] for a,b,t in images[key] if t=='Cart - Front']
            extra=[[a,b,t] for a,b,t in images[key] if t!='Cart - Front']
            if scans:physical.append([title,scans]);titles[title]=titles.get(title,0)|8
            if extra:supplemental.append([title,extra]);titles[title]=titles.get(title,0)|16
        catalog['platforms'][platform]={'repo':repo,'revision':tree['sha'],'games':sorted(titles.items())}
        catalog['physical'][platform]=physical
        catalog['supplemental'][platform]=supplemental
        print(platform,len(titles),'matching titles;',len(physical),'physical cartridge scans',flush=True)
    if args.systems=='gameboy':
        catalog['gameboy_added']='2026-10-19'
        catalog['gameboy_launchbox_snapshot']='2026-10-09'
    else:
        data=ET.parse(args.fbneo_dat).getroot()
        catalog['arcade_aliases']={g.attrib['name']:g.findtext('description') for g in data.findall('game') if g.attrib.get('isbios')!='yes'}
        catalog['arcade_bios']=[g.attrib['name'] for g in data.findall('game') if g.attrib.get('isbios')=='yes']
        catalog['additional_systems_added']='2026-10-22'
        catalog['additional_launchbox_snapshot']='2026-10-09'
    (here/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,separators=(',',':')),encoding='utf-8')

if __name__=='__main__':main()
