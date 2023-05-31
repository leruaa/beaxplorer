use std::{cmp::min, convert::TryFrom, fmt::Display};

use convert_case::{Case, Casing};
use futures::future::try_join_all;
use js_sys::Array;
use serde::Deserialize;
use tsify::Tsify;
use types::{path::ToPath, DeserializeOwned};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{
    app::App,
    fetcher::fetch,
    sort::{Paginate, SortBy},
    DeserializeError, PathArray,
};

#[derive(Tsify, Deserialize)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct RangeSettings {
    pub page_index: usize,
    pub page_size: usize,
    pub sort_id: String,
    pub sort_desc: bool,
}

#[derive(Tsify, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct RangeInput {
    pub settings: RangeSettings,
    pub plural: String,
    pub kind: RangeKind,
}

#[derive(Tsify, Deserialize)]
#[tsify(from_wasm_abi)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
pub enum RangeKind {
    Integers,
    Strings,
    Epoch { number: u64 },
}

#[derive(Debug)]
pub enum ModelId {
    AsString(String),
    AsU64(u64),
}

impl Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelId::AsString(s) => write!(f, "{s}"),
            ModelId::AsU64(u) => write!(f, "{u}"),
        }
    }
}

impl From<String> for ModelId {
    fn from(value: String) -> Self {
        ModelId::AsString(value)
    }
}

impl TryFrom<ModelId> for String {
    type Error = DeserializeError;

    fn try_from(value: ModelId) -> Result<Self, Self::Error> {
        match value {
            ModelId::AsString(s) => Ok(s),
            ModelId::AsU64(u) => Err(DeserializeError::InvalidModelId(ModelId::AsU64(u))),
        }
    }
}

impl From<u64> for ModelId {
    fn from(value: u64) -> Self {
        ModelId::AsU64(value)
    }
}

impl TryFrom<ModelId> for u64 {
    type Error = DeserializeError;

    fn try_from(value: ModelId) -> Result<Self, Self::Error> {
        match value {
            ModelId::AsString(s) => Err(DeserializeError::InvalidModelId(ModelId::AsString(s))),
            ModelId::AsU64(u) => Ok(u),
        }
    }
}

pub async fn get_paths<M>(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<PathArray, JsValue>
where
    M: ToPath,
    M::Id: Clone + TryFrom<ModelId> + Into<JsValue>,
{
    get_range(app, input, total_count)
        .await
        .map(|ids| {
            ids.into_iter()
                .flat_map(M::Id::try_from)
                .map(|id| {
                    vec![
                        id.clone().into(),
                        JsValue::from(M::to_path(&app.base_url(), &id)),
                    ]
                })
                .map(|vec| vec.into_iter().collect::<Array>())
                .collect::<Array>()
                .unchecked_into()
        })
        .map_err(Into::into)
}

pub async fn get_range(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<Vec<ModelId>, DeserializeError> {
    if input.settings.sort_id == "default" && !matches!(input.kind, RangeKind::Epoch { .. }) {
        Ok(get_default_range(&input, total_count))
    } else {
        match &input.kind {
            RangeKind::Integers => fetch_range::<u64>(app, input, total_count).await,
            RangeKind::Strings => fetch_range::<String>(app, input, total_count).await,
            RangeKind::Epoch { number } => Ok(get_epoch_range(*number)),
        }
    }
}

pub fn get_default_range(input: &RangeInput, total_count: usize) -> Vec<ModelId> {
    let range = if input.settings.sort_desc {
        let end = total_count - input.settings.page_index * input.settings.page_size;
        let start = end.saturating_sub(input.settings.page_size);
        start..end
    } else {
        let start = input.settings.page_index * input.settings.page_size;
        let end = min(start + input.settings.page_size, total_count);
        start..end
    };

    if input.settings.sort_desc {
        range
            .map(|x| ModelId::AsU64(x as u64))
            .rev()
            .collect::<Vec<_>>()
    } else {
        range
            .map(|x: usize| ModelId::AsU64(x as u64))
            .collect::<Vec<_>>()
    }
}

pub fn get_epoch_range(epoch: u64) -> Vec<ModelId> {
    let epoch = epoch as usize;
    let range = epoch * 32..(epoch + 1) * 32;

    range.map(|id| ModelId::AsU64(id as u64)).collect()
}

async fn fetch_range<Id>(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<Vec<ModelId>, DeserializeError>
where
    Id: DeserializeOwned + Into<ModelId>,
{
    let sort_by = SortBy::new(input.settings.sort_id, input.settings.sort_desc);

    let mut futures = vec![];
    for page_number in Paginate::new(
        total_count,
        input.settings.page_index + 1,
        input.settings.page_size,
        &sort_by,
    ) {
        let url = format!(
            "{}/{}/s/{}/{}.msg",
            app.base_url(),
            input.plural,
            sort_by.clone().id.to_case(Case::Snake),
            page_number
        );
        futures.push(fetch::<Vec<Id>>(url));
    }

    let mut ids = try_join_all(futures)
        .await?
        .into_iter()
        .flatten()
        .map(|id| id.into())
        .collect::<Vec<_>>();

    if sort_by.desc {
        ids.reverse()
    }

    Ok(ids)
}
