use std::cmp::min;

use futures::future::try_join_all;
use js_sys::{Array, Error, Promise};
use types::{DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    fetcher::{fetch, fetch_all},
    sort::{Paginate, SortBy},
    DeserializeError,
};

pub fn page<V: DeserializeOwned + Serialize>(
    base_url: String,
    page_index: usize,
    page_size: usize,
    sort_id: String,
    sort_desc: bool,
    total_count: usize,
) -> Promise {
    let sort_by = SortBy::new(sort_id, sort_desc);

    future_to_promise(async move {
        let range = match sort_by.id.as_str() {
            "default" => {
                let range = if sort_by.desc {
                    let end = total_count - page_index * page_size;
                    let start = end.checked_sub(page_size).unwrap_or(0);
                    start..end
                } else {
                    let start = page_index * page_size;
                    let end = min(start + page_size, total_count);
                    start..end
                };

                let result = if sort_by.desc {
                    range.map(|x| x as u64).rev().collect()
                } else {
                    range.map(|x| x as u64).collect()
                };

                Ok(result)
            }
            _ => {
                let mut futures = vec![];
                for page_number in Paginate::new(total_count, page_index + 1, page_size, &sort_by) {
                    let url = format!(
                        "{}/s/{}/{}.msg",
                        base_url.clone(),
                        sort_by.clone().id,
                        page_number
                    );
                    futures.push(fetch::<Vec<u64>>(url));
                }

                let range = try_join_all(futures)
                    .await
                    .map(|x| x.into_iter().flatten().collect());

                if sort_by.desc {
                    let skip = if page_index == 0 {
                        0 as usize
                    } else {
                        10 - total_count as usize % 10
                    };
                    range.map(|x: Vec<u64>| {
                        x.into_iter()
                            .rev()
                            .skip(skip)
                            .take(page_size as usize)
                            .collect()
                    })
                } else {
                    range
                }
            }
        };

        match range {
            Ok(range) => {
                let url = base_url + "/{}.msg";
                let result = get_paginated::<V>(url, range).await;

                match result {
                    Ok(value) => Ok(value),
                    Err(err) => Err(Error::new(&err.to_string()).into()),
                }
            }
            Err(err) => Err(Error::new(&err.to_string()).into()),
        }
    })
}

async fn get_paginated<V: DeserializeOwned + Serialize>(
    base_url: String,
    range: Vec<u64>,
) -> Result<JsValue, DeserializeError> {
    fetch_all::<V, u64>(base_url, range)
        .await?
        .into_iter()
        .map(|v| JsValue::from_serde(&v).map_err(|err| DeserializeError::SerdeJson(err)))
        .collect::<Result<Vec<JsValue>, DeserializeError>>()
        .map(|x| x.into_iter().collect::<Array>().into())
}
