var buildVersion = "{{buildVersion}}"
var cacheName = 'leptos-pwa';
var filesToCache = [
  './',
  './index.html',
  './manifest.json',
  './public/radicals.csv',
  './public/tmp03bqpcha.wav',
  './icon-256.png',
  './leptos-tutorial-' + buildVersion + '_bg.wasm',
  './leptos-tutorial-' + buildVersion + '.js',
];


/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function(e) {
    console.log("Installing service-worker for build", buildVersion);
    preCache = async () => {
        get_cache().then(function (cache) {
            cache.keys().then(function (requests) {
                for (let request of requests) {
                    cache.delete(request);
                }
            });
            cache.addAll(filesToCache.map(url => new Request(url, {credentials: 'same-origin'})));
        })
    };
    e.waitUntil(preCache);
});

self.addEventListener('message', function (messageEvent) {
    if (messageEvent.data === "skipWaiting") {
        console.log("Service-worker received skipWaiting event", buildVersion);
        self.skipWaiting();
    }
});

/* Serve cached content when offline */
self.addEventListener('fetch', function(e) {
    e.respondWith(
        caches.match(e.request).then(function(response) {
            return response || fetch(e.request);
        })
    );
});

async function get_cache() {
  return caches.open(cacheName);
}

async function cache_then_network(request) {
    const cache = await get_cache();
    return cache.match(request).then(
        (cache_response) => {
            if (!cache_response) {
                return fetch_from_network(request, cache);
            } else {
                return cache_response;
            }
        },
        (reason) => {
            return fetch_from_network(request, cache);
        }
    );
}

function fetch_from_network(request, cache) {
    return fetch(request).then(
        (net_response) => {
            return net_response;
        },
        (reason) => {
            console.error("Network fetch rejected. Falling back to ./index.html. Reason: ", reason);
            return cache.match("./index.html").then(function (cache_root_response) {
                return cache_root_response;
            });
        }
    )
}