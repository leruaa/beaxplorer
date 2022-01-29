use js_sys::Promise;
use types::epoch::{
    EpochExtendedModel, EpochExtendedModelWithId, EpochExtendedView, EpochModel, EpochModelWithId,
    EpochView, EpochsMeta,
};
use types::persisting_path::PersistingPathWithId;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{fetcher::fetch, page::page, to_js};

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

    pub fn get(&self, epoch: u64) -> Promise {
        let epoch_url = format!(
            "{}/{}",
            self.base_url.clone(),
            EpochModelWithId::to_path(epoch)
        );
        let extended_epoch_url = format!(
            "{}/{}",
            self.base_url.clone(),
            EpochExtendedModelWithId::to_path(epoch)
        );

        future_to_promise(async move {
            let model = fetch::<EpochModel>(epoch_url).await?;
            let extended_model = fetch::<EpochExtendedModel>(extended_epoch_url).await?;
            to_js::<EpochExtendedView>(&(epoch, model, extended_model).into()).map_err(Into::into)
        })
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<EpochModel, EpochView>(
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
