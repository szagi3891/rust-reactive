use wasm_bindgen::prelude::*;

mod dom_event;
mod fetch;
// mod set_interval;
mod events;
mod element_wrapper;
mod dom_utils;
mod dom_driver_browser;
mod dom_driver_interval;

#[wasm_bindgen(module = "/jsdriver/out/jsdriver.js")]
extern "C" {
    type MyClass;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;

    type DomDriverJsInterval;

    #[wasm_bindgen(constructor)]
    pub fn new(callback: &Closure<dyn Fn(u64)>) -> DomDriverJsInterval;

    #[wasm_bindgen(method)]
    pub fn set_interval(this: &DomDriverJsInterval, duration: u32, callback_id: u64) -> u32;

    #[wasm_bindgen(method)]
    pub fn clear_interval(this: &DomDriverJsInterval, timer_id: u32);
    
}

pub use dom_driver_browser::DomDriverBrowser;
