use js_sys::{Array, ArrayBuffer};
use types::attestation::AttestationModel;
use types::block::{BlockExtendedModel, BlockModel};
use types::committee::CommitteeModel;
use types::meta::Meta;
use types::path::ToPath;
use types::vote::VoteModel;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::attestations::AttestationView;
use crate::views::blocks::{BlockExtendedView, BlockPaths, BlockView};
use crate::views::committees::CommitteeView;
use crate::views::votes::VoteView;
use crate::{deserialize, AttestationArray, CommitteeArray, PathArray, VoteArray};

use crate::to_js;

#[wasm_bindgen(js_name = "getBlock")]
pub fn get_block(buffer: ArrayBuffer, block: u64) -> Result<BlockView, JsValue> {
    let model = deserialize::<BlockModel>(buffer)?;
    Ok(BlockView::from((block, model)))
}

#[wasm_bindgen(js_name = "getBlockPaths")]
pub fn get_block_paths(app: &App, slot: u64) -> BlockPaths {
    BlockPaths {
        block: BlockModel::to_path(&app.base_url(), &slot),
        block_extended: BlockExtendedModel::to_path(&app.base_url(), &slot),
        committees: CommitteeModel::to_path(&app.base_url(), &slot),
        votes: VoteModel::to_path(&app.base_url(), &slot),
        attestations: AttestationModel::to_path(&app.base_url(), &slot),
    }
}

#[wasm_bindgen(js_name = "getBlockRangePaths")]
pub async fn get_block_range_paths(
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
    slot: u64,
) -> Result<BlockExtendedView, JsValue> {
    let model = deserialize::<BlockModel>(model_buffer)?;
    let extended_model = deserialize::<BlockExtendedModel>(extended_model_buffer)?;
    Ok(BlockExtendedView::from((slot, model, extended_model)))
}

#[wasm_bindgen(js_name = "getCommittees")]
pub async fn get_committees(committees_buffer: ArrayBuffer) -> Result<CommitteeArray, JsValue> {
    deserialize::<Vec<CommitteeModel>>(committees_buffer)?
        .into_iter()
        .map(CommitteeView::from)
        .map(|v| to_js(&v))
        .collect::<Result<Array, _>>()
        .map(|a| a.unchecked_into())
        .map_err(Into::into)
}

#[wasm_bindgen(js_name = "getVotes")]
pub async fn get_votes(votes_buffer: ArrayBuffer) -> Result<VoteArray, JsValue> {
    deserialize::<Vec<VoteModel>>(votes_buffer)?
        .into_iter()
        .map(VoteView::from)
        .map(|v| to_js(&v))
        .collect::<Result<Array, _>>()
        .map(|a| a.unchecked_into())
        .map_err(Into::into)
}

#[wasm_bindgen(js_name = "getAttestations")]
pub async fn get_attestations(
    attestations_buffer: ArrayBuffer,
) -> Result<AttestationArray, JsValue> {
    deserialize::<Vec<AttestationModel>>(attestations_buffer)?
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
