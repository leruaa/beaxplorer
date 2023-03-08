use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct App {
    base_url: String,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String) -> App {
        App {
            base_url: base_url + "/data",
        }
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }
}
