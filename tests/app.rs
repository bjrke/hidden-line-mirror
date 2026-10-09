use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn start_hook_installs() {
    hidden_line::main_js().expect("start hook should install");
}
