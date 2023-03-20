use js_sys::Array;
use types::attestation::{AttestationModel, AttestationModelsWithId};
use types::block::{
    BlockExtendedModel, BlockExtendedModelWithId, BlockModel, BlockModelWithId, BlocksMeta,
};
use types::committee::{CommitteeModel, CommitteeModelsWithId};
use types::path::ToPath;
use types::vote::{VoteModel, VoteModelsWithId};
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::views::attestations::AttestationView;
use crate::views::blocks::{BlockExtendedView, BlockView};
use crate::views::committees::CommitteeView;
use crate::views::votes::VoteView;
use crate::{AttestationArray, CommitteeArray, VoteArray};

use crate::{fetcher::fetch, to_js};

#[wasm_bindgen(js_name = "getBlock")]
pub async fn get_block(app: &App, block: u64) -> Result<BlockView, JsValue> {
    let block_url = BlockModelWithId::to_path(&app.base_url(), &block);

    let model = fetch::<BlockModel>(block_url).await?;
    Ok(BlockView::from((block, model)))
}

#[wasm_bindgen(js_name = "getBlockExtended")]
pub async fn get_block_extended(app: &App, block: u64) -> Result<BlockExtendedView, JsValue> {
    let block_url = BlockModelWithId::to_path(&app.base_url(), &block);
    let extended_block_url = BlockExtendedModelWithId::to_path(&app.base_url(), &block);

    let model = fetch::<BlockModel>(block_url).await?;
    let extended_model = fetch::<BlockExtendedModel>(extended_block_url).await?;
    Ok(BlockExtendedView::from((block, model, extended_model)))
}

#[wasm_bindgen(js_name = "getCommittees")]
pub async fn get_committees(app: &App, block: u64) -> Result<CommitteeArray, JsValue> {
    let committees_url = CommitteeModelsWithId::to_path(&app.base_url(), &block);

    fetch::<Vec<CommitteeModel>>(committees_url)
        .await?
        .into_iter()
        .map(CommitteeView::from)
        .map(|v| to_js(&v))
        .collect::<Result<Array, _>>()
        .map(|a| a.unchecked_into())
        .map_err(Into::into)
}

#[wasm_bindgen(js_name = "getVotes")]
pub async fn get_votes(app: &App, block: u64) -> Result<VoteArray, JsValue> {
    let votes_url = VoteModelsWithId::to_path(&app.base_url(), &block);

    fetch::<Vec<VoteModel>>(votes_url)
        .await?
        .into_iter()
        .map(VoteView::from)
        .map(|v| to_js(&v))
        .collect::<Result<Array, _>>()
        .map(|a| a.unchecked_into())
        .map_err(Into::into)
}

#[wasm_bindgen(js_name = "getAttestations")]
pub async fn get_attestations(app: &App, block: u64) -> Result<AttestationArray, JsValue> {
    let attestations_url = AttestationModelsWithId::to_path(&app.base_url(), &block);

    fetch::<Vec<AttestationModel>>(attestations_url)
        .await?
        .into_iter()
        .map(AttestationView::from)
        .map(|v| to_js(&v))
        .collect::<Result<Array, _>>()
        .map(|a| a.unchecked_into())
        .map_err(Into::into)
}

#[wasm_bindgen(js_name = "getBlockMeta")]
pub async fn get_block_meta(app: &App) -> Result<BlocksMeta, JsValue> {
    let meta_url = BlocksMeta::to_path(&app.base_url(), &());

    fetch::<BlocksMeta>(meta_url).await.map_err(Into::into)
}
