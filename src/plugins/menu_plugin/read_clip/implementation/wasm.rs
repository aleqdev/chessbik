use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js/read_clip.js")]
extern "C" {
    pub fn read_clip() -> String;
}