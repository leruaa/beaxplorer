use std::cmp::min;

use convert_case::{Case, Casing};
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use types::DeserializeOwned;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    app::App,
    fetcher::fetch,
    sort::{Paginate, SortBy},
    to_js_with_large_numbers_as_bigints,
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

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(untagged)]
pub enum Range {
    Integers {
        #[tsify(type = "bigint[]")]
        range: Vec<u64>,
    },
    Strings {
        range: Vec<String>,
    },
}

impl From<Range> for JsValue {
    fn from(val: Range) -> Self {
        to_js_with_large_numbers_as_bigints(&val).unwrap()
    }
}

#[wasm_bindgen(js_name = "getRange")]
pub async fn get_range(app: &App, input: RangeInput, total_count: usize) -> Result<Range, JsValue> {
    if input.settings.sort_id == "default" && !matches!(input.kind, RangeKind::Epoch { .. }) {
        Ok(get_default_range(&input, total_count))
    } else {
        match &input.kind {
            RangeKind::Integers => fetch_range::<u64>(app, input, total_count)
                .await
                .map(|r| Range::Integers { range: r }),
            RangeKind::Strings => fetch_range::<String>(app, input, total_count)
                .await
                .map(|r| Range::Strings { range: r }),
            RangeKind::Epoch { number } => Ok(get_epoch_range(*number)),
        }
    }
}

pub fn get_default_range(input: &RangeInput, total_count: usize) -> Range {
    let range = if input.settings.sort_desc {
        let end = total_count - input.settings.page_index * input.settings.page_size;
        let start = end.saturating_sub(input.settings.page_size);
        start..end
    } else {
        let start = input.settings.page_index * input.settings.page_size;
        let end = min(start + input.settings.page_size, total_count);
        start..end
    };

    let result = if input.settings.sort_desc {
        range.map(|x| x as u64).rev().collect::<Vec<_>>()
    } else {
        range.map(|x| x as u64).collect::<Vec<_>>()
    };

    Range::Integers { range: result }
}

pub fn get_epoch_range(epoch: u64) -> Range {
    let range = epoch * 32..(epoch + 1) * 32;

    Range::Integers {
        range: range.collect(),
    }
}

async fn fetch_range<Id: DeserializeOwned>(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<Vec<Id>, JsValue> {
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

    let mut range = try_join_all(futures)
        .await
        .map(|x| x.into_iter().flatten().collect());

    if sort_by.desc {
        let skip = if input.settings.page_index == 0 {
            0_usize
        } else {
            10 - total_count % 10
        };
        let page_size = input.settings.page_size;
        range = range.map(|x: Vec<Id>| x.into_iter().rev().skip(skip).take(page_size).collect())
    }

    range.map_err(Into::into)
}
