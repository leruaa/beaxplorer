use js_sys::Promise;
use types::validator::{ValidatorModel, ValidatorView, ValidatorsMeta};
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
        let url = base_url + "/data/validators";
        let meta = fetch(format!("{}/meta.msg", url)).await?;

        Ok(Validators::new(url, meta).into())
    }

    pub fn get(&self, validator: String) -> Promise {
        by_id::<ValidatorModel, ValidatorView>(self.base_url.clone(), validator)
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
            page_index,
            page_size,
            sort_id,
            sort_desc,
            self.meta.count.clone(),
        )
    }

    pub fn meta(&self) -> Result<JsValue, JsValue> {
        return to_js(&self.meta).map_err(Into::into);
    }
}
