import("../pkg/index.js").then(
    wasm => {
        const hiddenLine = wasm.lets_go(document.getElementsByTagName("svg").item(0));
        document.body.addEventListener("click", () => {
            hiddenLine.on_click()
        }, true);
    }
).catch(console.error);
