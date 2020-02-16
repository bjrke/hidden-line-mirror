import("../pkg/index.js").then(
    wasm => {
        const formula = document.getElementById("formula");
        const formulaForm = document.getElementById("formulaForm");
        const submitFormula = document.getElementById("submitFormula");

        if (location.hash && location.hash.length > 1) {
            formula.value = decodeURIComponent(location.hash.substr(1));
        } else {
            formula.value =
                "var h = Math.sqrt(x * x + y * y);\nreturn 25 * Math.cos(h) / (2 + h);";
        }
        const autoSize = () => {
            formula.style.height = "auto";
            formula.style.height = formula.scrollHeight + "px";
            submitFormula.style.visibility = "visible";
        }

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
        hiddenLine.set_function(getF());
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
        }, true);



        formulaForm.addEventListener("submit", (e) => {
            const f = getF();
            if (f) {
                location.hash = encodeURIComponent(formula.value);
                submitFormula.style.visibility = "hidden";
                svg.focus();
                hiddenLine.set_function(f);
            }
            e.preventDefault();
            return false;
        });

        svg.focus();
    }
).catch(console.error);
