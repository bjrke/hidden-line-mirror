import * as wasm from "../pkg/index.js";

let instance = null;

self.onmessage = ({ data: msg }) => {
    if (msg.type === "scene") {
        instance = new wasm.Worker();
        instance.set_scene(
            new Float32Array(msg.points),
            new Uint32Array(msg.triangles)
        );
        self.postMessage({ type: "scene_ok" });
    } else if (msg.type === "compute") {
        let records;
        try {
            records = instance
                ? instance
                      .compute(new Float32Array(msg.camera), msg.rank, msg.count)
                      .slice()
                : new Float32Array();
        } catch (e) {
            console.error(e);
            records = new Float32Array();
        }
        self.postMessage(
            { type: "records", gen: msg.gen, records },
            [records.buffer]
        );
    }
};

self.postMessage({ type: "ready" });
