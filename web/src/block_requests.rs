use types::{
    block_request::{BlockRequestModel, BlockRequestModelWithId},
    path::ToPath,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    app::App,
    fetcher::{fetch, fetch_meta},
    views::{block_requests::BlockRequestView, meta::MetaView},
};

#[wasm_bindgen(js_name = "getBlockRequest")]
pub async fn get_block_request(app: &App, root: String) -> Result<BlockRequestView, JsValue> {
    let block_request_url = BlockRequestModelWithId::to_path(&app.base_url(), &root);

    let model = fetch::<BlockRequestModel>(block_request_url).await?;
    Ok(BlockRequestView::from((root, model)))
}

#[wasm_bindgen(js_name = "getBlockRequestMeta")]
pub async fn get_block_request_meta(app: &App) -> Result<MetaView, JsValue> {
    fetch_meta::<BlockRequestModel>(app).await
}
