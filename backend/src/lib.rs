use wasm_bindgen::prelude::*;
use std::fmt::Display;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn toggle_mode() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Window not found.");
    let mut main = window
                .document()
                .unwrap()
                .get_element_by_id("main")
                .unwrap();


    main.class_list()
        .toggle("dark")
        .unwrap();

    Ok(())
}