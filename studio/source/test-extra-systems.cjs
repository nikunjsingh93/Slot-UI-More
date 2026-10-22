// Run with Playwright/JSZip available in NODE_PATH. Optional SLOT_TEST_BROWSER.
// No ROM contents or real game artwork are used by the deterministic tests.
const {chromium}=require('playwright');
const JSZip=require('jszip');
const fs=require('fs'),path=require('path'),assert=require('assert');
const html=path.resolve(process.argv[2]||path.join(__dirname,'../Slot-Auto-Artwork-Studio-v5.html'));
const output=path.resolve(process.argv[3]||'studio-extra-test');fs.mkdirSync(output,{recursive:true});
const png=fs.readFileSync(path.join(__dirname,'../../docs/screenshots/snes.png'));
const names={NES:'Super Mario Bros. (World).nes',MD:'Sonic The Hedgehog (USA, Europe).md',SMS:'Alex Kidd in Miracle World (USA, Europe).sms',GG:'Sonic The Hedgehog (USA, Europe, Brazil).gg',PCE:"Bonk 3 - Bonk's Big Adventure (USA).pce",ARCADE:'sf2.zip'};
(async()=>{
 const browser=await chromium.launch({headless:true,...(process.env.SLOT_TEST_BROWSER?{executablePath:process.env.SLOT_TEST_BROWSER}:{})});
 try {
  const page=await browser.newPage({viewport:{width:1280,height:1000},acceptDownloads:true}),errors=[];
  page.on('pageerror',e=>errors.push(String(e)));
  await page.route(/https:\/\/(?:raw\.githubusercontent\.com|wsrv\.nl)\//,route=>route.fulfill({status:200,contentType:'image/png',body:png,headers:{'access-control-allow-origin':'*'}}));
  await page.goto('file:///'+html.replace(/\\/g,'/'));
  const result=await page.evaluate(async names=>{
   const reads=[];const files=new Map();
   for(const [system,name] of Object.entries(names))files.set('Slot/Games/'+system+'/'+name,()=>{reads.push(name);throw Error('ROM read prohibited');});
   files.set('Slot/Games/ARCADE/neogeo.zip',()=>{throw Error('BIOS must be ignored');});
   files.set('Slot/Games/MD/not-a-game.exe',()=>{throw Error('Wrong extension');});
   const rows=await scanFiles(files);await loadRows(rows,null,'Synthetic test');
   return {reads,rows:games.map(g=>({p:g.platform,name:g.stem,state:g.state,match:g.match?.title,size:g.label?[g.label.width,g.label.height]:null,face:g.label?[faceCanvas(g.platform,g.label).width,faceCanvas(g.platform,g.label).height]:null}))};
  },names);
  assert.equal(result.rows.length,6);assert.deepEqual(result.reads,[]);
  const sizes={NES:[190,245],MD:[264,151],SMS:[252,230],GG:[206,168],PCE:[276,139],ARCADE:[204,124]};
  for(const row of result.rows){assert.equal(row.state,'Ready',JSON.stringify(row));assert.deepEqual(row.size,sizes[row.p]);}
  assert(result.rows.find(r=>r.p==='ARCADE').match.includes('Street Fighter II'));
  await page.screenshot({path:path.join(output,'extra-systems-preview.png'),fullPage:true});
  const pending=page.waitForEvent('download');await page.locator('#zip').click();const download=await pending;await download.saveAs(path.join(output,'labels.zip'));
  const zip=await JSZip.loadAsync(fs.readFileSync(path.join(output,'labels.zip')));
  for(const [system,name] of Object.entries(names))assert(zip.file('Labels/'+system+'/'+name.slice(0,name.lastIndexOf('.'))+'.png'));
  // Nested folder detection must also work with ONLY a newly supported system.
  assert.equal(await page.evaluate(async()=> (await scanFiles(new Map([['Outer/Slot/Games/NES/Test.nes',()=>{throw Error('ROM read');}]])))[0].platform),'NES');
  // Old label protection, fallback and direct folder saving.
  const protection=await page.evaluate(async png=>{
   const entries=new Map([['Games/NES/Super Mario Bros. (World).nes',()=>{throw Error('ROM read');}],['Labels/NES/Super Mario Bros. (World).png',async()=>new Blob([Uint8Array.from(atob(png),c=>c.charCodeAt(0))],{type:'image/png'})]]);
   const rows=await scanFiles(entries);return rows.map(g=>({existing:g.existing,state:g.state}));
  },png.toString('base64'));assert(protection[0].existing);assert.equal(protection[0].state,'Existing label kept');
  const writes=await page.evaluate(async()=>{
   window.__extraWrites=[];
   const dir=prefix=>({async getDirectoryHandle(name){return dir(prefix+'/'+name);},async getFileHandle(name,options){if(!options?.create)throw new DOMException('Missing','NotFoundError');return {async createWritable(){return {async write(blob){window.__extraWrites.push([prefix+'/'+name,blob.size]);},async close(){},async abort(){}};}};}});
   rootHandle={async queryPermission(){return 'granted';},async getDirectoryHandle(name){return dir(name);}};updateSummary();return true;
  });assert(writes);await page.locator('#save').click();await page.waitForFunction(()=>!busy&&window.__extraWrites.length===6);
  await page.setViewportSize({width:390,height:844});assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
  assert.deepEqual(errors,[]);fs.writeFileSync(path.join(output,'validation.json'),JSON.stringify({passed:true,...result,checks:['six-system automatic matching','FBNeo short names','BIOS hidden','no ROM reads','PNG dimensions','exact ZIP names','direct save','existing labels preserved','new-system-only nested prefix','mobile layout']},null,2));
  console.log('PASS new systems: automatic artwork, arcade aliases, BIOS filtering, no ROM reads, labels ZIP/direct save, prefix detection, existing-label protection and mobile layout');
 } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
