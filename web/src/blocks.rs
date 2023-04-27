use js_sys::{Array, ArrayBuffer};
use types::attestation::{AttestationModel, AttestationModelsWithId};
use types::block::{BlockExtendedModel, BlockModel};
use types::committee::{CommitteeModel, CommitteeModelsWithId};
use types::meta::Meta;
use types::path::ToPath;
use types::vote::{VoteModel, VoteModelsWithId};
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::attestations::AttestationView;
use crate::views::blocks::{BlockExtendedView, BlockView};
use crate::views::committees::CommitteeView;
use crate::views::votes::VoteView;
use crate::{deserialize, AttestationArray, CommitteeArray, PathArray, VoteArray};

use crate::{fetcher::fetch, to_js};

#[wasm_bindgen(js_name = "getBlock")]
pub fn get_block(buffer: ArrayBuffer, block: u64) -> Result<BlockView, JsValue> {
    let model = deserialize::<BlockModel>(buffer)?;
    Ok(BlockView::from((block, model)))
}

#[wasm_bindgen(js_name = "getBlockPaths")]
pub async fn get_epoch_paths(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<PathArray, JsValue> {
    get_paths::<BlockModel>(app, input, total_count).await
}

#[wasm_bindgen(js_name = "getBlockExtended")]
pub fn get_block_extended(
    model_buffer: ArrayBuffer,
    extended_model_buffer: ArrayBuffer,
    block: u64,
) -> Result<BlockExtendedView, JsValue> {
    let model = deserialize::<BlockModel>(model_buffer)?;
    let extended_model = deserialize::<BlockExtendedModel>(extended_model_buffer)?;
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

#[wasm_bindgen(js_name = "getBlockMetaPath")]
pub fn get_block_meta_path(app: &App) -> JsValue {
    Meta::to_path::<BlockModel>(&app.base_url()).into()
}
