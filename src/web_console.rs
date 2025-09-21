#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
    browser_log("[Init] Panic hook installed");
}

#[cfg(target_arch = "wasm32")]
pub fn browser_log(message: &str) {
    log(message);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn browser_log(_message: &str) {}
