import * as wasm from "../pkg/index.js";

const workerCount = Math.max(
    4,
    (navigator.hardwareConcurrency || 2) * 2
);

const workers = [];
for (let i = 0; i < workerCount; i++) {
    workers.push(new Worker(new URL("./worker.js", import.meta.url)));
}

function once(worker, type) {
    return new Promise(resolve => {
        const handler = e => {
            if (e.data.type === type) {
                worker.removeEventListener("message", handler);
                resolve(e.data);
            }
        };
        worker.addEventListener("message", handler);
    });
}

const workersReady = Promise.all(
    workers.map(worker => once(worker, "ready"))
);

let sceneReady = false;
let drawing = false;
let queued = false;
let gen = 0;

function scheduleDraw() {
    if (!sceneReady || drawing) {
        queued = true;
        return;
    }
    drawing = true;
    computeFrame()
        .catch(console.error)
        .finally(() => {
            drawing = false;
            if (queued) {
                queued = false;
                scheduleDraw();
            }
        });
}

function computeFrame() {
    const myGen = ++gen;
    const camera = hiddenLine.camera();
    return Promise.all(
        workers.map((worker, rank) => {
            const result = once(worker, "records");
            worker.postMessage({
                type: "compute",
                gen: myGen,
                rank,
                count: workers.length,
                camera
            });
            return result;
        })
    ).then(parts => {
        if (myGen !== gen) return;
        let total = 0;
        for (const part of parts) total += part.records.length;
        const merged = new Float32Array(total);
        let offset = 0;
        for (const part of parts) {
            merged.set(part.records, offset);
            offset += part.records.length;
        }
        hiddenLine.draw_records(merged);
    });
}

async function rebuild() {
    const points = hiddenLine.mesh_points();
    const triangles = hiddenLine.mesh_triangles();
    await workersReady;
    await Promise.all(
        workers.map(worker => {
            const done = once(worker, "scene_ok");
            worker.postMessage({ type: "scene", points, triangles });
            return done;
        })
    );
    sceneReady = true;
    scheduleDraw();
}

const svg = document.getElementsByTagName("svg").item(0);
const hiddenLine = wasm.lets_go(svg, scheduleDraw, rebuild);
rebuild();

const buildInfo = document.getElementById("buildInfo");
if (buildInfo) {
    buildInfo.textContent = __BUILD_INFO__;
}

const licenseLink = document.getElementById("licenseLink");
const licenseDialog = document.getElementById("licenseDialog");
if (licenseLink && licenseDialog) {
    const licenseText = document.getElementById("licenseText");
    licenseLink.addEventListener("click", async event => {
        event.preventDefault();
        if (!licenseText.textContent) {
            try {
                const response = await fetch(licenseLink.href);
                licenseText.textContent = await response.text();
            } catch (e) {
                console.error(e);
            }
        }
        licenseDialog.showModal();
    });
    document.getElementById("licenseClose").addEventListener("click", () => {
        licenseDialog.close();
    });
    licenseDialog.addEventListener("click", event => {
        if (event.target === licenseDialog) {
            licenseDialog.close();
        }
    });
}
