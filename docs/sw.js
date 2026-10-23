// Offline-Cache: die App liegt komplett in index.html
const C="meldermap-1.13.1-1792781762568";
const FILES=["./","index.html","manifest.webmanifest","icon-192.png","icon-512.png","maskable-192.png","maskable-512.png","apple-touch-icon.png"];
self.addEventListener("install",e=>{e.waitUntil(caches.open(C).then(c=>c.addAll(FILES)).then(()=>self.skipWaiting()));});
self.addEventListener("activate",e=>{e.waitUntil(caches.keys().then(ks=>Promise.all(ks.filter(k=>k!==C&&k!=="meldermap-inbox").map(k=>caches.delete(k)))).then(()=>self.clients.claim()));});
// Android „Teilen → MelderMap“: Datei ablegen, App liest sie beim Start (?inbox=1)
self.addEventListener("fetch",e=>{if(e.request.method!=="POST"||!/share-target/.test(e.request.url))return;e.respondWith((async()=>{const fd=await e.request.formData();const c=await caches.open("meldermap-inbox");let i=0;for(const f of fd.getAll("file"))await c.put("./inbox/"+Date.now()+"-"+(i++),new Response(f));return Response.redirect("./?inbox=1",303);})());});
self.addEventListener("fetch",e=>{if(e.request.method!=="GET")return;e.respondWith(fetch(e.request).then(r=>{const cp=r.clone();caches.open(C).then(c=>c.put(e.request,cp));return r;}).catch(()=>caches.match(e.request,{ignoreSearch:true}).then(r=>r||caches.match("index.html"))));});
