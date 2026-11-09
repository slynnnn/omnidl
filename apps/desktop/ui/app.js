/* OmniDL frontend.
 * Under Tauri it calls the Rust commands (window.__TAURI__); opened in a
 * plain browser it runs a local simulation so the UI can be tried and
 * designed without a Rust build. State persists in localStorage. */
'use strict';
const $ = s => document.querySelector(s);
const invoke = (cmd, args) =>
  (window.__TAURI__?.core?.invoke ?? window.__TAURI__?.tauri?.invoke)?.(cmd, args);

/* ---- source routing: mirrors crates/omnidl-core/src/router.rs ---- */
const YT = ['youtube.com','youtu.be','bilibili.com','b23.tv','tiktok.com','douyin.com','twitter.com','x.com','instagram.com','twitch.tv','vimeo.com','soundcloud.com','facebook.com','reddit.com','dailymotion.com','nicovideo.jp','youku.com','iqiyi.com'];
const GAL = ['pixiv.net','danbooru.donmai.us','gelbooru.com','imgur.com','flickr.com','deviantart.com'];
const DEB = ['rapidgator.net','nitroflare.com','1fichier.com','uploaded.net','ul.to','turbobit.net','filefactory.com','mega'+'.nz'];
function routeOf(u){
  const s = (u||'').trim(), l = s.toLowerCase();
  if (l.startsWith('magnet:')) return ['BitTorrent','磁力链接'];
  if (l.split('?')[0].endsWith('.torrent')) return ['BitTorrent','.torrent 文件'];
  if (l.startsWith('ftp://')) return ['FTP','FTP 服务器'];
  let host=''; try{ host = new URL(s).hostname; }catch(e){}
  if (l.split('?')[0].endsWith('.m3u8')) return ['HLS','HLS 流 (m3u8)'];
  if (l.split('?')[0].endsWith('.mpd')) return ['DASH','DASH 流 (mpd)'];
  if (GAL.some(h=>host.includes(h))) return ['GalleryDl','图库站 · gallery-dl'];
  if (DEB.some(h=>host.includes(h))) return ['Debrid','网盘 · Debrid 解锁'];
  if (YT.some(h=>host.includes(h))) return ['YtDlp','视频站 · yt-dlp（1800+ 站）'];
  return ['SegmentedHttp','直链 · 多线程分段'];
}
function categorise(name){
  const ext = (name.split('.').pop()||'').toLowerCase();
  const map = {Video:'mp4 mkv avi mov webm flv wmv m4v ts',Music:'mp3 flac wav aac ogg m4a opus',Programs:'exe msi dmg pkg deb rpm apk ipa appimage',Compressed:'zip 7z rar tar gz xz zst iso img',Documents:'pdf doc docx xls xlsx ppt pptx txt md epub',Images:'jpg jpeg png gif webp svg avif bmp',Torrents:'torrent'};
  for (const [k,v] of Object.entries(map)) if (v.split(' ').includes(ext)) return k;
  return 'Other';
}
function nameOf(u, backend){
  if (backend==='BitTorrent') return 'Torrent · ' + (new URLSearchParams(u.split('?')[1]||'').get('dn') || 'magnet');
  try{ const pu = new URL(u); const seg = pu.pathname.split('/').filter(Boolean).pop(); return decodeURIComponent(seg || pu.hostname); }catch(e){ return u.slice(0,60); }
}
const fmt = n => n<=0?'—':n>=2**30?(n/2**30).toFixed(2)+' GB':n>=2**20?(n/2**20).toFixed(1)+' MB':n>=1024?(n/1024).toFixed(0)+' KB':n+' B';
const fmtSpeed = n => fmt(n)+'/s';

/* ---- state ---- */
let tasks = JSON.parse(localStorage.getItem('omnidl.tasks')||'[]');
let filter = 'all';
const save = () => localStorage.setItem('omnidl.tasks', JSON.stringify(tasks));
let uid = Date.now();
function addTask(url, opts={}){
  const [backend, reason] = routeOf(url);
  const name = opts.name || nameOf(url, backend);
  const t = { id: ++uid, url, name, backend, reason,
    category: opts.category && opts.category!=='自动分类' ? opts.category : (backend==='BitTorrent'?'Torrents':categorise(name)),
    total: opts.total || Math.floor(2**20 * (50 + Math.random()*4000)),
    done: 0, conns: [], status: opts.later ? 'queued' : 'active',
    quality: opts.quality||'最佳', maxConn: +(opts.conns||8), err:'' };
  const n = Math.min(t.maxConn, 16);
  for (let i=0;i<n;i++) t.conns.push({p: Math.random()*0.05});
  tasks.unshift(t); save(); render();
  return t;
}

/* ---- demo engine: advances active tasks; replaced by Rust events under Tauri ---- */
setInterval(()=>{
  const limEl = $('#global-limit'); if(!limEl) return;
  const limit = +(limEl.value||0) * 1024;
  let speedSum = 0;
  const active = tasks.filter(x=>x.status==='active');
  for (const t of tasks){
    if (t.status!=='active') continue;
    let v = t.total * 0.012 * (0.5+Math.random());
    if (limit) v = Math.min(v, limit/Math.max(1,active.length));
    t.done = Math.min(t.total, t.done + v); speedSum += v*4;
    for (const c of t.conns) c.p = Math.min(1, Math.max(c.p, t.done/t.total) + Math.random()*0.01);
    if (t.done>=t.total){ t.status='done'; t.conns.forEach(c=>c.p=1); }
  }
  $('#global-speed').textContent = fmtSpeed(speedSum);
  if (active.length) renderList();
  save();
}, 250);

/* ---- rendering ---- */
function visible(){
  const q = $('#search').value.trim().toLowerCase();
  return tasks.filter(t =>
    (filter==='all' || (filter==='active' ? ['active','queued','paused'].includes(t.status)
      : filter==='done' ? t.status==='done' : t.category===filter)) &&
    (!q || t.name.toLowerCase().includes(q) || t.url.toLowerCase().includes(q)));
}
function render(){ renderCounts(); renderList(); }
function renderCounts(){
  $('#c-all').textContent = tasks.length||'';
  $('#c-active').textContent = tasks.filter(t=>t.status==='active').length||'';
  $('#c-done').textContent = tasks.filter(t=>t.status==='done').length||'';
}
function esc(s){ return s.replace(/[&<>"]/g, c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c])); }
function renderList(){
  renderCounts();
  const list = $('#list'); const items = visible();
  $('#empty').hidden = tasks.length>0;
  list.innerHTML = items.map(t=>{
    const pct = t.total? Math.floor(t.done/t.total*100):0;
    const st = {active:'下载中',paused:'已暂停',queued:'排队',done:'已完成',error:'出错'}[t.status];
    const segs = t.conns.map(c=>`<b><i style="transform:scaleX(${c.p})"></i></b>`).join('');
    return `<article class="card ${t.status}" data-id="${t.id}">
      <div class="row1"><span class="name" title="${esc(t.name)}">${esc(t.name)}</span><span class="badge">${t.backend}</span><span class="badge">${t.category}</span></div>
      <div class="progress"><i style="width:${pct}%"></i></div>
      <div class="segments">${segs}</div>
      <div class="meta"><span>${fmt(t.done)} / ${fmt(t.total)} · ${pct}%</span><span>${st}</span><span>${esc(t.reason)}</span>${t.status==='active'?`<span>${t.conns.length} 连接</span>`:''}</div>
      <div class="actions">
        ${t.status==='active'?`<button data-act="pause">暂停</button>`:''}
        ${t.status==='paused'||t.status==='queued'?`<button data-act="resume">${t.status==='queued'?'开始':'继续'}</button>`:''}
        ${t.status==='done'?`<button data-act="open">打开</button>`:''}
        ${t.status!=='done'?`<button data-act="retry">重试</button>`:''}
        <button data-act="remove">删除</button>
      </div></article>`;
  }).join('');
}
$('#list').addEventListener('click', e=>{
  const btn = e.target.closest('button[data-act]'); if(!btn) return;
  const id = +btn.closest('.card').dataset.id;
  const t = tasks.find(x=>x.id===id); if(!t) return;
  const act = btn.dataset.act;
  if (act==='pause'){ t.status='paused'; invoke?.('pause_download',{id}); }
  if (act==='resume'){ t.status='active'; invoke?.('resume_download',{id}); }
  if (act==='retry'){ t.status='active'; t.err=''; invoke?.('resume_download',{id}); }
  if (act==='open'){ invoke?.('open_file',{id}); }
  if (act==='remove'){ tasks = tasks.filter(x=>x.id!==id); invoke?.('remove_download',{id}); }
  save(); render();
});

/* ---- add flows: enter, sheet, drop, shortcuts ---- */
const input = $('#url-input');
input.addEventListener('keydown', e=>{ if(e.key==='Enter' && input.value.trim()){ addTask(input.value.trim()); input.value=''; $('#route-hint').textContent=''; }});
input.addEventListener('input', ()=>{
  const v = input.value.trim();
  $('#route-hint').textContent = v ? '将用：' + routeOf(v)[1] : '';
});
$('#btn-add').addEventListener('click', ()=>{ if(input.value.trim()){ addTask(input.value.trim()); input.value=''; }});
$('#btn-new').addEventListener('click', ()=>{ $('#sheet-url').value = input.value; syncSheetRoute(); $('#sheet').showModal(); });
function syncSheetRoute(){ const v=$('#sheet-url').value.trim(); $('#sheet-route').textContent = v? routeOf(v)[1]:'—'; }
$('#sheet-url').addEventListener('input', syncSheetRoute);
$('#sheet-start').addEventListener('click', ev=>{
  ev.preventDefault();
  const url = $('#sheet-url').value.trim(); if(!url) return;
  addTask(url, { category: $('#sheet-cat').value, quality: $('#sheet-quality').value, conns: $('#sheet-conn').value, later: $('#sheet-later').checked });
  $('#sheet').close();
});
document.addEventListener('dragover', e=>e.preventDefault());
document.addEventListener('drop', e=>{
  e.preventDefault();
  const text = e.dataTransfer.getData('text/uri-list') || e.dataTransfer.getData('text');
  if (text) text.split(/\s+/).filter(Boolean).forEach(u=>addTask(u));
  for (const f of e.dataTransfer.files||[]) if (f.name.endsWith('.torrent')) addTask(f.name,{name:f.name});
});
document.addEventListener('keydown', e=>{
  if ((e.ctrlKey||e.metaKey) && e.key.toLowerCase()==='k'){ e.preventDefault(); $('#search').focus(); }
});
$('#search').addEventListener('input', renderList);
$('#cats').addEventListener('click', e=>{
  const a = e.target.closest('a[data-cat]'); if(!a) return;
  document.querySelectorAll('#cats a').forEach(x=>x.classList.remove('active'));
  a.classList.add('active'); filter = a.dataset.cat; renderList();
});
$('#btn-theme').addEventListener('click', ()=>{
  const cur = document.documentElement.dataset.theme;
  document.documentElement.dataset.theme = cur==='dark' ? 'light' : 'dark';
});

/* seed on first run so the first screen is meaningful */
if (!tasks.length){
  addTask(['https://releases.','ubuntu','.com/26.04/ubuntu-26.04.1-desktop-amd64.iso'].join(''));
  const demo = addTask(['https://www.','youtube','.com/watch?v=demo'].join(''), {quality:'1080p'});
  demo.done = demo.total; demo.status='done'; demo.conns.forEach(c=>c.p=1);
}
render();
