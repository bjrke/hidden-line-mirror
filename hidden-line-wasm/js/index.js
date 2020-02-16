import("../pkg/index.js").then(
    wasm => {
        const hiddenLine = wasm.lets_go(document.getElementsByTagName("svg").item(0));
        document.body.addEventListener("click", () => {
            hiddenLine.on_click()
        }, true);

        let h = false;
        document.body.addEventListener("keydown", e => {
            const key = e.key;
            if (key.length != 1) return;
            if (h) return;
            h = true;
            hiddenLine.on_key(key);
            window.setTimeout(() => { h = false; });
        });
    }
).catch(console.error);
