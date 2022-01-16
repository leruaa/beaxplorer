use js_sys::Promise;
use types::{meta::EpochsMeta, views::EpochView};
use wasm_bindgen::prelude::*;

use crate::{fetcher::fetch, get::by_id, page::page, to_js};

#[wasm_bindgen]
pub struct Epochs {
    base_url: String,
    meta: EpochsMeta,
}

#[wasm_bindgen]
impl Epochs {
    fn new(base_url: String, meta: EpochsMeta) -> Epochs {
        Epochs { base_url, meta }
    }

    #[wasm_bindgen]
    pub async fn build(base_url: String) -> Result<Epochs, JsValue> {
        let url = base_url + "/data/epochs";
        let meta = fetch(format!("{}/meta.msg", url)).await?;

        Ok(Epochs::new(url, meta).into())
    }

    pub fn get(&self, epoch: String) -> Promise {
        by_id::<EpochView>(self.base_url.clone(), epoch)
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<EpochView>(
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
