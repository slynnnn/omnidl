'use strict';
const enabledEl = document.getElementById('enabled');
const siteOffEl = document.getElementById('site-off');
chrome.storage.local.get({ enabled: true, disabledSites: [] }, prefs => {
  enabledEl.checked = prefs.enabled;
  chrome.tabs.query({ active: true, currentWindow: true }, tabs => {
    try { siteOffEl.checked = prefs.disabledSites.includes(new URL(tabs[0].url).hostname); } catch (e) {}
  });
});
enabledEl.addEventListener('change', () => chrome.storage.local.set({ enabled: enabledEl.checked }));
siteOffEl.addEventListener('change', () => {
  chrome.tabs.query({ active: true, currentWindow: true }, tabs => {
    let host = ''; try { host = new URL(tabs[0].url).hostname; } catch (e) {}
    chrome.storage.local.get({ disabledSites: [] }, prefs => {
      const set = new Set(prefs.disabledSites);
      siteOffEl.checked ? set.add(host) : set.delete(host);
      chrome.storage.local.set({ disabledSites: [...set] });
    });
  });
});
document.getElementById('open-app').addEventListener('click', () => {
  try { chrome.runtime.sendNativeMessage('one.omnidl.app', { type: 'open' }, () => void chrome.runtime.lastError); } catch (e) {}
});
