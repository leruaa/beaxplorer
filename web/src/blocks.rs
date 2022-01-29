use js_sys::Promise;
use types::block::{
    BlockExtendedModel, BlockExtendedModelWithId, BlockExtendedView, BlockModel, BlockModelWithId,
    BlockView, BlocksMeta,
};
use types::persisting_path::PersistingPathWithId;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{fetcher::fetch, page::page, to_js};

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

    pub fn get(&self, block: u64) -> Promise {
        let block_url = format!(
            "{}/{}",
            self.base_url.clone(),
            BlockModelWithId::to_path(block)
        );
        let extended_block_url = format!(
            "{}/{}",
            self.base_url.clone(),
            BlockExtendedModelWithId::to_path(block)
        );

        future_to_promise(async move {
            let model = fetch::<BlockModel>(block_url).await?;
            let extended_model = fetch::<BlockExtendedModel>(extended_block_url).await?;
            to_js::<BlockExtendedView>(&(block, model, extended_model).into()).map_err(Into::into)
        })
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<BlockModel, BlockView>(
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
