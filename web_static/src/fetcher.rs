use bytes::Buf;
use futures::future::try_join_all;
use types::DeserializeOwned;

use crate::DeserializeError;

pub async fn fetch<T: DeserializeOwned>(url: String) -> Result<T, DeserializeError> {
    let response = reqwest::get(url).await?;

    rmp_serde::from_read::<_, T>(response.bytes().await?.reader()).map_err(Into::into)
}

pub async fn fetch_all<T: DeserializeOwned, S: ToString>(
    url: String,
    range: Vec<S>,
) -> Result<Vec<T>, DeserializeError> {
    let mut futures = vec![];

    for id in range {
        futures.push(fetch::<T>(url.replace("{}", &id.to_string())));
    }

    try_join_all(futures).await
}
