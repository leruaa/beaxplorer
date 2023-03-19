use types::{
    good_peer::{GoodPeerModel, GoodPeerModelWithId, GoodPeersMeta},
    path::ToPath,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{app::App, fetcher::fetch, views::good_peers::GoodPeerView};

#[wasm_bindgen(js_name = "getGoodPeer")]
pub async fn get_block_request(app: &App, id: String) -> Result<GoodPeerView, JsValue> {
    let good_peer_url = GoodPeerModelWithId::to_path(&app.base_url(), &id);

    let model = fetch::<GoodPeerModel>(good_peer_url).await?;
    Ok(GoodPeerView::from((id, model)))
}

#[wasm_bindgen(js_name = "getGoodPeerMeta")]
pub async fn get_good_peer_meta(app: &App) -> Result<GoodPeersMeta, JsValue> {
    let meta_url = GoodPeersMeta::to_path(&app.base_url(), &());

    fetch::<GoodPeersMeta>(meta_url).await.map_err(Into::into)
}
