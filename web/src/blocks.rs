use js_sys::Promise;
use types::attestation::{AttestationModel, AttestationsModelWithId};
use types::block::{
    BlockExtendedModel, BlockExtendedModelWithId, BlockModel, BlockModelWithId, BlocksMeta,
};
use types::committee::{CommitteeModel, CommitteesModelWithId};
use types::meta::Meta;
use types::path::ToPath;
use types::vote::{VoteModel, VotesModelWithId};
use wasm_bindgen::prelude::*;

use crate::views::attestations::AttestationView;
use crate::views::blocks::{BlockExtendedView, BlockView};
use crate::views::commitees::CommitteeView;
use crate::views::votes::VoteView;
use crate::{fetcher::fetch, page::page, to_js};

#[wasm_bindgen]
pub struct Blocks {}

#[wasm_bindgen]
impl Blocks {
    pub async fn get(base_url: String, block: u64) -> Result<JsValue, JsValue> {
        let block_url = BlockModelWithId::to_path(&*base_url, block);
        let extended_block_url = BlockExtendedModelWithId::to_path(&*base_url, block);

        let model = fetch::<BlockModel>(block_url).await?;
        let extended_model = fetch::<BlockExtendedModel>(extended_block_url).await?;
        to_js::<BlockExtendedView>(&(block, model, extended_model).into()).map_err(Into::into)
    }

    pub async fn committees(base_url: String, block: u64) -> Result<JsValue, JsValue> {
        let committees_url = CommitteesModelWithId::to_path(&*base_url, block);
        let commitees = fetch::<Vec<CommitteeModel>>(committees_url)
            .await?
            .into_iter()
            .map(CommitteeView::from)
            .collect::<Vec<_>>();

        to_js(&commitees).map_err(Into::into)
    }

    pub async fn votes(base_url: String, block: u64) -> Result<JsValue, JsValue> {
        let votes_url = VotesModelWithId::to_path(&*base_url, block);
        let votes = fetch::<Vec<VoteModel>>(votes_url)
            .await?
            .into_iter()
            .map(VoteView::from)
            .collect::<Vec<_>>();

        to_js(&votes).map_err(Into::into)
    }

    pub async fn attestations(base_url: String, block: u64) -> Result<JsValue, JsValue> {
        let attestations_url = AttestationsModelWithId::to_path(&*base_url, block);

        let r = fetch::<Vec<AttestationModel>>(attestations_url)
            .await?
            .into_iter()
            .map(AttestationView::from)
            .collect::<Vec<_>>();

        to_js(&r).map_err(Into::into)
    }

    pub fn page(
        base_url: String,
        page_index: usize,
        page_size: usize,
        total_count: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<BlockModel, BlockView>(
            base_url,
            "blocks".to_string(),
            page_index,
            page_size,
            sort_id,
            sort_desc,
            total_count,
        )
    }

    pub async fn meta(base_url: String) -> Result<JsValue, JsValue> {
        let meta = fetch::<BlocksMeta>(BlocksMeta::to_path(&*base_url)).await?;

        to_js(&meta).map_err(Into::into)
    }
}
