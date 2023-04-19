use types::{
    good_peer::{GoodPeerModel, GoodPeerModelWithId},
    path::ToPath,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    app::App,
    fetcher::{fetch, fetch_meta},
    views::{good_peers::GoodPeerView, meta::MetaView},
};

#[wasm_bindgen(js_name = "getGoodPeer")]
pub async fn get_block_request(app: &App, id: String) -> Result<GoodPeerView, JsValue> {
    let good_peer_url = GoodPeerModelWithId::to_path(&app.base_url(), &id);

    let model = fetch::<GoodPeerModel>(good_peer_url).await?;
    Ok(GoodPeerView::from((id, model)))
}

#[wasm_bindgen(js_name = "getGoodPeerMeta")]
pub async fn get_good_peer_meta(app: &App) -> Result<MetaView, JsValue> {
    fetch_meta::<GoodPeerModel>(app).await
}
