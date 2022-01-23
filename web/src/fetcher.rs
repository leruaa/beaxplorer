use bytes::Buf;
use futures::future::try_join_all;
use types::DeserializeOwned;

use crate::DeserializeError;

pub async fn fetch<T: DeserializeOwned>(url: String) -> Result<T, DeserializeError> {
    let response = reqwest::get(url).await?;

    rmp_serde::from_read::<_, T>(response.bytes().await?.reader()).map_err(Into::into)
}

pub async fn fetch_all<T: DeserializeOwned>(
    url: String,
    range: Vec<u64>,
) -> Result<Vec<(u64, T)>, DeserializeError> {
    let mut futures = vec![];

    for id in range {
        let url = url.replace("{}", &id.to_string());
        futures.push(async move { fetch::<T>(url).await.map(|x| (id, x)) });
    }

    try_join_all(futures).await
}
