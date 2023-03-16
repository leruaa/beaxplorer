use types::{
    block_request::{BlockRequestModel, BlockRequestModelWithId, BlockRequestsMeta},
    path::ToPath,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{app::App, fetcher::fetch, views::block_requests::BlockRequestView};

#[wasm_bindgen(js_name = "getBlockRequest")]
pub async fn get_block_request(app: &App, root: String) -> Result<BlockRequestView, JsValue> {
    let block_request_url = BlockRequestModelWithId::to_path(&app.base_url(), &root);

    let model = fetch::<BlockRequestModel>(block_request_url).await?;
    Ok(BlockRequestView::from((root, model)))
}

#[wasm_bindgen(js_name = "getBlockRequestMeta")]
pub async fn get_block_request_meta(app: &App) -> Result<BlockRequestsMeta, JsValue> {
    let meta_url = BlockRequestsMeta::to_path(&app.base_url(), &());

    fetch::<BlockRequestsMeta>(meta_url)
        .await
        .map_err(Into::into)
}
