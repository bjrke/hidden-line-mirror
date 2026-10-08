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

function openDialog(dialog) {
    if (dialog.open) {
        return;
    }
    dialog.showModal();
    dialog.focus();
    dialog.scrollTop = 0;
}

function setupDialog(link, dialog, closeId, onOpen) {
    link.addEventListener("click", async event => {
        event.preventDefault();
        if (onOpen) {
            try {
                await onOpen();
            } catch (e) {
                console.error(e);
            }
        }
        openDialog(dialog);
    });
    document.getElementById(closeId).addEventListener("click", () => {
        dialog.close();
    });
    dialog.addEventListener("click", event => {
        if (event.target === dialog) {
            dialog.close();
        }
    });
    dialog.addEventListener("wheel", event => event.stopPropagation());
}

const licenseLink = document.getElementById("licenseLink");
const licenseDialog = document.getElementById("licenseDialog");
if (licenseLink && licenseDialog) {
    const licenseText = document.getElementById("licenseText");
    setupDialog(licenseLink, licenseDialog, "licenseClose", async () => {
        if (!licenseText.textContent) {
            const response = await fetch(licenseLink.href);
            licenseText.textContent = await response.text();
        }
    });
}

const helpDialog = document.getElementById("helpDialog");
if (helpDialog) {
    setupDialog(document.getElementById("helpLink"), helpDialog, "helpClose");
    document.addEventListener("keydown", event => {
        if (!["?", "h", "H"].includes(event.key) || event.target === document.getElementById("formula")) {
            return;
        }
        event.preventDefault();
        if (helpDialog.open) {
            helpDialog.close();
        } else {
            openDialog(helpDialog);
        }
    });
}
