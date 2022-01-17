use js_sys::Promise;
use types::{meta::BlocksMeta, views::BlockView};
use wasm_bindgen::prelude::*;

use crate::{fetcher::fetch, get::by_id, page::page, to_js};

#[wasm_bindgen]
pub struct Blocks {
    base_url: String,
    meta: BlocksMeta,
}

#[wasm_bindgen]
impl Blocks {
    fn new(base_url: String, meta: BlocksMeta) -> Blocks {
        Blocks { base_url, meta }
    }

    #[wasm_bindgen]
    pub async fn build(base_url: String) -> Result<Blocks, JsValue> {
        let url = base_url + "/data/blocks";
        let meta = fetch(format!("{}/meta.msg", url)).await?;

        Ok(Blocks::new(url, meta).into())
    }

    pub fn get(&self, block: String) -> Promise {
        by_id::<BlockView>(self.base_url.clone(), block)
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<BlockView>(
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
