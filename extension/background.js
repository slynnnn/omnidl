/* OmniDL extension (MV3, Chrome/Edge/Brave/Vivaldi/Firefox).
 * - Intercepts browser downloads and hands them to the app through
 *   Native Messaging (host: one.omnidl.app), including cookies/referrer.
 * - Right-click: "Download with OmniDL" on links, images, videos.
 * - Hold Alt while clicking to let the browser handle a download. */
const HOST = 'one.omnidl.app';
let altHeld = false;

function sendToApp(payload) {
  try {
    chrome.runtime.sendNativeMessage(HOST, payload, resp => {
      if (chrome.runtime.lastError) console.debug('OmniDL app not reachable', chrome.runtime.lastError.message);
    });
  } catch (e) { /* app not installed: browser download proceeds */ }
}

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({ id: 'omnidl-link', title: 'Download with OmniDL', contexts: ['link'] });
  chrome.contextMenus.create({ id: 'omnidl-media', title: 'Download with OmniDL', contexts: ['image', 'video', 'audio'] });
});

chrome.contextMenus.onClicked.addListener(info => {
  const url = info.linkUrl || info.srcUrl;
  if (url) sendToApp({ type: 'add', url, referrer: info.pageUrl });
});

chrome.downloads.onCreated.addListener(item => {
  if (altHeld) return; // user override: keep it in the browser
  chrome.storage.local.get({ enabled: true, disabledSites: [] }, prefs => {
    if (!prefs.enabled) return;
    let host = ''; try { host = new URL(item.finalUrl || item.url).hostname; } catch (e) {}
    if (prefs.disabledSites.includes(host)) return;
    chrome.cookies.getAll({ url: item.url }, cookies => {
      sendToApp({ type: 'add', url: item.finalUrl || item.url, filename: item.filename,
                  referrer: item.referrer, cookies });
      chrome.downloads.cancel(item.id); // the app takes it from here
    });
  });
});

chrome.runtime.onMessage.addListener(msg => {
  if (msg.type === 'video-found') sendToApp({ type: 'add', url: msg.url, referrer: msg.pageUrl });
  if (msg.type === 'alt') altHeld = !!msg.held;
});
