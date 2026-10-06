import registerMouse from "./mouse";

const workerCount = Math.max(
    4,
    (navigator.hardwareConcurrency || 2) * 2
);

const workers = [];
for (let i = 0; i < workerCount; i++) {
    workers.push(new Worker(new URL("./worker.js", import.meta.url)));
}

import("../pkg/index.js").then(
    wasm => {
        const formula = document.getElementById("formula");
        const formulaForm = document.getElementById("formulaForm");
        const submitFormula = document.getElementById("submitFormula");

        if (location.hash && location.hash.length > 1) {
            formula.value = decodeURIComponent(location.hash.substr(1));
        } else {
            formula.value =
                "let h=0;\n" +
                "const step=Math.PI/36;\n" +
                "for (let a=0; a<Math.PI; a+=step) {\n" +
                "  const c=Math.cos(a);\n" +
                "  const s=Math.sin(a);\n" +
                "  const xd=x*c-y*s;\n" +
                "  const yd=x*s+y*c;\n" +
                "  h += Math.cos(a*Math.sqrt(xd*xd*25+yd*yd*100))/(Math.PI+a);\n" +
                "}\n" +
                "return h/10;";
        }

        const autoSize = () => {
            formula.style.height = "auto";
            formula.style.height = formula.scrollHeight + "px";
            submitFormula.style.visibility = "visible";
        };

        formula.style.height = "auto";
        formula.style.height = formula.scrollHeight + "px";
        submitFormula.style.visibility = "hidden";
        formula.addEventListener("input", autoSize, false);

        function getF() {
            try {
                return eval("((x,y)=>{" + formula.value + "})");
            } catch (e) {
                alert(e);
                return null;
            }
        }

        const svg = document.getElementsByTagName("svg").item(0);
        const hiddenLine = wasm.lets_go(svg);

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

        async function broadcastScene() {
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
        }

        let gen = 0;
        let drawing = false;
        let queued = false;
        let sceneReady = false;

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

        function applyFormula(f) {
            if (!f) return;
            hiddenLine.set_function(f);
            sceneReady = false;
            broadcastScene().then(() => {
                sceneReady = true;
                scheduleDraw();
            });
        }

        applyFormula(getF());

        svg.addEventListener(
            "keydown",
            e => {
                const key = e.key;
                if (key.length != 1) return;
                if (hiddenLine.on_key(key)) {
                    scheduleDraw();
                }
            },
            true
        );

        formulaForm.addEventListener("submit", e => {
            const f = getF();
            if (f) {
                location.hash = encodeURIComponent(formula.value);
                submitFormula.style.visibility = "hidden";
                svg.focus();
                applyFormula(f);
            }
            e.preventDefault();
            return false;
        });

        svg.focus();
        registerMouse(svg);
    }
).catch(console.error);
