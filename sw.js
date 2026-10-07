/* PrintCraft service worker: cache the app shell so it starts instantly offline. */
const CACHE = 'printcraft-v0.2.1-4';
const CORE = [
  './',
  './index.html',
  './manifest.webmanifest',
  './privacy.html',
  './printcraft-web-b585eb4bdc7cf69.js',
  './printcraft-web-b585eb4bdc7cf69_bg.wasm',
  './icons/icon-192.png',
  './icons/icon-512.png',
  './mobile/',
  './mobile/index.html',
  './mobile/manifest.webmanifest',
  './mobile/vendor/pdf-lib.min.js',
];

self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll(CORE)).then(() => self.skipWaiting()));
});

self.addEventListener('activate', (e) => {
  e.waitUntil(
    caches.keys()
      .then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', (e) => {
  const url = new URL(e.request.url);
  if (url.origin !== self.location.origin) return;
  // Hashed assets (wasm/js/icons): cache-first, they never change.
  const immutable = /\.(wasm|js|png)$/.test(url.pathname);
  e.respondWith(
    (async () => {
      const cache = await caches.open(CACHE);
      if (immutable) {
        const hit = await cache.match(e.request);
        if (hit) return hit;
      }
      try {
        const fresh = await fetch(e.request);
        if (fresh.ok) cache.put(e.request, fresh.clone());
        return fresh;
      } catch (err) {
        const hit = await cache.match(e.request);
        if (hit) return hit;
        throw err;
      }
    })()
  );
});
