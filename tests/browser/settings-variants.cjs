// Product-specific settings contracts; no browser or third-party modules required.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const source=fs.readFileSync(path.join(__dirname,'../../web/app.js'),'utf8');
const html=fs.readFileSync(path.join(__dirname,'../../web/index.html'),'utf8');
function extract(start,end){const a=source.indexOf(start),b=source.indexOf(end,a);assert(a>=0&&b>a);return source.slice(a,b);}
const sections=['station','interfaces','beacon','mailbox','messages','yapp','ax25','application','web'].map(settingsKey=>({dataset:{settingsKey}}));
const form={querySelectorAll:()=>sections};
const control={checked:false,disabled:true},heading={},label={},hint={};
const requests=[];
const context=vm.createContext({window:{},state:{webDetails:{runtime:'unexpected'},catalog:{},language:'pl'},
  $:id=>({'settings-desktop-autostart':control,'settings-desktop-autostart-label':label,'settings-desktop-autostart-hint':hint}[id]),
  document:{querySelector:selector=>selector==='.settings-form'?form:heading},
  request:async route=>{requests.push(route);return{enabled:true};},
});
vm.runInContext(extract('function settingsTabItemsSeparated(){','\nsettingsTabItems=settingsTabItemsSeparated;')+
 extract('function settingsDesktopRuntime(){','\nsetInterval(syncSettingsRuntime,500);'),context);
const keys=Array.from(context.settingsTabItemsSeparated(),item=>item.key);
assert(!keys.includes('settings.notifications'));
assert.equal(keys.includes('settings.application'),true);
assert.equal(keys.includes('settings.web'),false);
context.syncNotificationChannels();
assert.equal(heading.textContent,'Powiadomienia');
context.state.language='en';context.syncNotificationChannels();assert.equal(heading.textContent,'Notifications');
assert(!html.match(/id="notification-[^"]*-telegram"/));
assert(source.includes("const channels=['in-app','native','sound'];"));
assert(!html.includes('telegram-settings'));
assert(!source.includes("request('telegram/"));
assert(!source.includes("request('mqtt/"));
assert(!source.includes("request('update/"));
assert(!source.includes('id="settings-web-bind"'));
(async()=>{await context.loadDesktopApplicationSettings();assert.deepEqual(requests,['desktop/autostart']);assert.equal(control.checked,true);assert.equal(control.disabled,false);assert(source.includes('control.checked=previous;throw exception;'));console.log('PASS: Terminal settings, native channel, autostart and no Station controls.');})().catch(e=>{console.error(e);process.exitCode=1;});
