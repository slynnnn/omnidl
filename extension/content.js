/* Adds a small "Download video" button over HTML5 players and reports
 * the media URL to the background worker. Deliberately minimal: one
 * button, positioned on the player, removable in the popup. */
(() => {
  const mark = v => {
    if (v.dataset.omnidl) return; v.dataset.omnidl = '1';
    const btn = document.createElement('button');
    btn.textContent = '↓ OmniDL';
    btn.style.cssText = 'position:absolute;top:10px;right:10px;z-index:2147483647;background:rgba(10,108,255,.92);color:#fff;border:none;border-radius:8px;padding:6px 12px;font:13px system-ui;cursor:pointer';
    btn.addEventListener('click', ev => {
      ev.stopPropagation();
      chrome.runtime.sendMessage({ type: 'video-found', url: location.href, pageUrl: location.href });
    });
    const wrap = v.parentElement;
    if (wrap) { wrap.style.position = wrap.style.position || 'relative'; wrap.appendChild(btn); }
  };
  const scan = () => document.querySelectorAll('video').forEach(mark);
  scan();
  new MutationObserver(scan).observe(document.documentElement, { childList: true, subtree: true });
  addEventListener('keydown', e => { if (e.key === 'Alt') chrome.runtime.sendMessage({ type: 'alt', held: true }); });
  addEventListener('keyup', e => { if (e.key === 'Alt') chrome.runtime.sendMessage({ type: 'alt', held: false }); });
})();
