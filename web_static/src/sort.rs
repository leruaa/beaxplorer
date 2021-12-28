use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SortBy {
    id: String,
    pub desc: bool,
}

#[wasm_bindgen]
impl SortBy {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, desc: bool) -> Self {
        SortBy { id, desc }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}
