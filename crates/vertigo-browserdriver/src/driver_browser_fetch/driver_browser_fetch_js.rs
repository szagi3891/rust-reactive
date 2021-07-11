use typescript_wasm_bindgen::typescript_wasm_bindgen;
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};

typescript_wasm_bindgen!(
    "crates/vertigo-browserdriver/src/driver_browser_fetch/driver_browser_fetch_js.ts",
    "crates/vertigo-browserdriver/src/driver_browser_fetch/driver_browser_fetch_js.js"
);

#[wasm_bindgen(module = "crates/vertigo-browserdriver/src/driver_browser_fetch/driver_browser_fetch_js.js")]
extern "C" {
    pub type DriverBrowserFetchJs2;

    #[wasm_bindgen(constructor)]
    pub fn new(callback: &Closure<dyn Fn(u64, bool, String)>) -> DriverBrowserFetchJs2;
    #[wasm_bindgen(method)]
    pub fn send_request(
        this: &DriverBrowserFetchJs2,
        request_id: u64,
        method: String,
        url: String,
        headers: String,
        body: Option<String>
    );
}

