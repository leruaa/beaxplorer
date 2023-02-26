use js_sys::Promise;
use types::path::ToPath;
use types::validator::{ValidatorModel, ValidatorModelWithId, ValidatorView, ValidatorsMeta};
use wasm_bindgen::prelude::*;

use crate::{fetcher::fetch, get::by_id, page::page, to_js};

#[wasm_bindgen]
pub struct Validators {
    base_url: String,
    meta: ValidatorsMeta,
}

#[wasm_bindgen]
impl Validators {
    fn new(base_url: String, meta: ValidatorsMeta) -> Validators {
        Validators { base_url, meta }
    }

    #[wasm_bindgen]
    pub async fn build(base_url: String) -> Result<Validators, JsValue> {
        let url = base_url + "/data";
        let meta = fetch(ValidatorsMeta::to_path(&*url, ())).await?;

        Ok(Validators::new(url, meta))
    }

    pub fn get(&self, validator: u64) -> Promise {
        let validator_url = ValidatorModelWithId::to_path(&*self.base_url, validator);
        by_id::<ValidatorModel, ValidatorView>(validator_url, validator)
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<ValidatorModel, ValidatorView>(
            self.base_url.clone(),
            "validators".to_string(),
            page_index,
            page_size,
            sort_id,
            sort_desc,
            self.meta.count,
        )
    }

    pub fn meta(&self) -> Result<JsValue, JsValue> {
        to_js(&self.meta).map_err(Into::into)
    }
}
