use std::cmp::min;

use futures::future::try_join_all;
use js_sys::{Array, BigUint64Array};
use types::DeserializeOwned;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{
    app::App,
    fetcher::fetch,
    sort::{Paginate, SortBy},
    StringArray,
};

#[wasm_bindgen(js_name = "getRangeAsNumbers")]
pub async fn get_range_as_numbers(
    app: &App,
    model_plural: String,
    page_index: usize,
    page_size: usize,
    sort_id: String,
    sort_desc: bool,
    total_count: usize,
) -> Result<BigUint64Array, JsValue> {
    get_range::<u64>(
        app,
        model_plural,
        page_index,
        page_size,
        sort_id,
        sort_desc,
        total_count,
    )
    .await
    .map(|a| BigUint64Array::from(a.as_slice()))
}

#[wasm_bindgen(js_name = "getRangeAsStrings")]
pub async fn get_range_as_strings(
    app: &App,
    model_plural: String,
    page_index: usize,
    page_size: usize,
    sort_id: String,
    sort_desc: bool,
    total_count: usize,
) -> Result<StringArray, JsValue> {
    get_range::<String>(
        app,
        model_plural,
        page_index,
        page_size,
        sort_id,
        sort_desc,
        total_count,
    )
    .await
    .map(|a| a.into_iter().map(JsValue::from).collect::<Array>())
    .map(|a| a.unchecked_into())
}

#[wasm_bindgen(js_name = "getDefaultRange")]
pub async fn get_default_range(
    page_index: usize,
    page_size: usize,
    sort_desc: bool,
    total_count: usize,
) -> BigUint64Array {
    let range = if sort_desc {
        let end = total_count - page_index * page_size;
        let start = end.saturating_sub(page_size);
        start..end
    } else {
        let start = page_index * page_size;
        let end = min(start + page_size, total_count);
        start..end
    };

    let result = if sort_desc {
        range.map(|x| x as u64).rev().collect::<Vec<_>>()
    } else {
        range.map(|x| x as u64).collect::<Vec<_>>()
    };

    BigUint64Array::from(result.as_slice())
}

pub async fn get_range<Id: DeserializeOwned>(
    app: &App,
    model_plural: String,
    page_index: usize,
    page_size: usize,
    sort_id: String,
    sort_desc: bool,
    total_count: usize,
) -> Result<Vec<Id>, JsValue> {
    let sort_by = SortBy::new(sort_id, sort_desc);

    let mut futures = vec![];
    for page_number in Paginate::new(total_count, page_index + 1, page_size, &sort_by) {
        let url = format!(
            "{}/{}/s/{}/{}.msg",
            app.base_url(),
            model_plural.clone(),
            sort_by.clone().id,
            page_number
        );
        futures.push(fetch::<Vec<Id>>(url));
    }

    let mut range = try_join_all(futures)
        .await
        .map(|x| x.into_iter().flatten().collect());

    if sort_by.desc {
        let skip = if page_index == 0 {
            0_usize
        } else {
            10 - total_count % 10
        };
        range = range.map(|x: Vec<Id>| x.into_iter().rev().skip(skip).take(page_size).collect())
    }

    range.map_err(Into::into)
}
