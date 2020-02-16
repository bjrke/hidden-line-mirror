import("../pkg/index.js").then(
    wasm => {
        const formula = document.getElementById("formula");
        const formulaForm = document.getElementById("formulaForm");

        const autoSize = () => {
            formula.style.height = "auto";
            formula.style.width = "auto";
            formula.style.height = formula.scrollHeight + "px";
            formula.style.width = formula.scrollWidth + "px";
        }

        autoSize();
        formula.addEventListener("input", autoSize, false);


        const svg = document.getElementsByTagName("svg").item(0);
        const hiddenLine = wasm.lets_go(svg);
        svg.addEventListener("click", () => {
            hiddenLine.on_click()
        }, true);

        let h = false;
        svg.addEventListener("keydown", e => {
            const key = e.key;
            if (key.length != 1) return;
            if (h) return;
            h = true;
            hiddenLine.on_key(key);
            window.setTimeout(() => { h = false; });
        });

        formulaForm.addEventListener("submit", (e) => {
            const f = eval("(" + formula.value + ")");
            hiddenLine.set_function(f);
            e.preventDefault();
        });

        svg.focus();
    }
).catch(console.error);
