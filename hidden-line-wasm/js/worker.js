let wasm = null;
let ready = false;
let pending = [];
let instance = null;

function handle(msg) {
    if (msg.type === "scene") {
        instance = new wasm.Worker();
        instance.set_scene(
            new Float32Array(msg.points),
            new Uint32Array(msg.triangles)
        );
        self.postMessage({ type: "scene_ok" });
    } else if (msg.type === "compute") {
        const records = instance
            .compute(new Float32Array(msg.camera), msg.rank, msg.count)
            .slice();
        self.postMessage(
            { type: "records", gen: msg.gen, records },
            [records.buffer]
        );
    }
}

self.onmessage = (event) => {
    if (!ready) {
        pending.push(event.data);
        return;
    }
    handle(event.data);
};

import("../pkg/index.js")
    .then((mod) => {
        wasm = mod;
        ready = true;
        for (const msg of pending) {
            handle(msg);
        }
        pending = [];
        self.postMessage({ type: "ready" });
    })
    .catch((error) => {
        self.postMessage({ type: "error", error: String(error) });
    });
