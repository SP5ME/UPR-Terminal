// CDP is enabled only by the manual Windows harness, using a fresh isolated profile without TNCs.
import assert from 'node:assert/strict';
import { writeFile, readFile, mkdir } from 'node:fs/promises';
import { resolve } from 'node:path';
const port = process.argv[2] || '19222';
const pages = await fetch(`http://127.0.0.1:${port}/json/list`).then(r => r.json());
const page = pages.find(p => p.type === 'page' && p.url.startsWith('http://127.0.0.1:'));
assert(page, 'Local panel not found');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((resolve, reject) => { ws.addEventListener('open', resolve, { once: true }); ws.addEventListener('error', reject, { once: true }); });
let id = 0;
const pending = new Map();
const exceptions = [];
ws.addEventListener('message', e => {
  const message = JSON.parse(e.data);
  if(message.method==='Runtime.exceptionThrown') exceptions.push(message.params.exceptionDetails);
  if (pending.has(message.id)) { pending.get(message.id)(message); pending.delete(message.id); }
});
async function rpc(method, params = {}) {
  const number = ++id;
  const response = new Promise(resolve => pending.set(number, resolve));
  ws.send(JSON.stringify({ id: number, method, params }));
  const timeout = setTimeout(() => { console.error(`CDP timeout: ${method}`); process.exit(1); }, 15000);
  const message = await response;
  clearTimeout(timeout);
  assert(!message.error, JSON.stringify(message.error));
  return message.result;
}
async function evaluate(expression) {
  const result = await rpc('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
  assert(!result.exceptionDetails, JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
try {
  await rpc('Runtime.enable');
  for (let i = 0; i < 40; i++) {
    if (await evaluate('document.readyState === "complete" && !!document.querySelector("#settings-desktop-autostart")')) break;
    await new Promise(r => setTimeout(r, 250));
  }
  assert.equal(await evaluate('window.__UPR_DESKTOP__'), true);
  assert.equal(await evaluate('!!document.querySelector("#view-terminal")'), true);
  const profile=await evaluate('request("state",{method:"GET"})');
  assert(!profile.callsign, 'Fresh test profile must have no station callsign');
  assert(profile.interfaces.length>0 && profile.interfaces.every(item=>!item.enabled), 'Manual UI test must not enable a TNC');
  assert.equal((await fetch(new URL('/api/state', page.url))).status, 200, 'Fresh local desktop profile has login disabled');
  await evaluate('loadSettings()');
  assert.equal(await evaluate('document.title'), 'UPR Terminal');
  assert.equal(await evaluate('!!document.querySelector("#settings-web-bind,#settings-mqtt-card,#settings-update-card,.telegram-settings")'), false);
  assert.equal(await evaluate('document.querySelectorAll(".notification-table thead th").length'), 4);
  assert.equal(await evaluate('!!document.querySelector("#settings-desktop-autostart")'), true);
  assert.equal(await evaluate('document.querySelectorAll(".form-section[data-settings-key=application]").length'), 1);
  assert.equal(await evaluate('document.querySelectorAll(".form-section[data-settings-key=web]").length'), 0);
  await evaluate('field("settings-callsign","N0CALL-1");field("settings-myname","Isolated split test")');
  assert.equal(await evaluate('saveAllConfiguration(false)'), true);
  assert.equal((await evaluate('request("config/details")')).myname, 'Isolated split test');
  await evaluate('loadSettings()');
  assert.equal(await evaluate('document.querySelectorAll(".form-section[data-settings-key=application]").length'), 1, 'Repeated loads must not duplicate preferences');
  await mkdir('target/desktop-downloads',{recursive:true});
  await rpc('Browser.setDownloadBehavior',{behavior:'allow',downloadPath:resolve('target/desktop-downloads')});
  // Exercise the same blob-based download mechanism used by the existing panel.
  await evaluate('download(new Blob(["desktop-download-check"],{type:"text/plain"}),"desktop-smoke.txt")');
  for(let i=0;i<30;i++){try{if(await readFile('target/desktop-downloads/desktop-smoke.txt','utf8')==='desktop-download-check')break;}catch{}await new Promise(r=>setTimeout(r,200));}
  assert.equal(await readFile('target/desktop-downloads/desktop-smoke.txt','utf8'),'desktop-download-check');
  await evaluate('request("state",{method:"GET"}).then(render)');
  const screen = await rpc('Page.captureScreenshot', { format: 'png' });
  await writeFile('target/desktop-smoke.png', Buffer.from(screen.data, 'base64'));
  assert.deepEqual(exceptions,[], 'No JavaScript errors during initialization and settings save');
  console.log('PASS: native WebView panel, isolated profile with disabled TNC, settings save and blob download');
} catch (error) {
  console.error(JSON.stringify(exceptions));
  console.error(await evaluate('JSON.stringify({sections:[...document.querySelectorAll(".form-section")].map(x=>x.dataset.settingsKey),split:document.querySelector(".settings-form")?.dataset.applicationWebSplit})'));
  console.error(await evaluate('JSON.stringify({url:location.href,ready:document.readyState,panel:!!document.querySelector("#view-terminal"),desktop:!!window.__UPR_DESKTOP__})'));
  throw error;
} finally { ws.close(); }
