use bytes::Buf;
use futures::future::try_join_all;
use types::{persisting_path::PersistingPathWithId, DeserializeOwned};

use crate::DeserializeError;

pub async fn fetch<T: DeserializeOwned>(url: String) -> Result<T, DeserializeError> {
    let response = reqwest::get(url).await?;

    rmp_serde::from_read::<_, T>(response.bytes().await?.reader()).map_err(Into::into)
}

pub async fn fetch_all<T>(
    base_url: String,
    range: Vec<u64>,
) -> Result<Vec<(u64, T)>, DeserializeError>
where
    T: DeserializeOwned,
    (u64, T): PersistingPathWithId<u64>,
{
    let mut futures = vec![];

    for id in range {
        let url = <(u64, T)>::to_path(&*base_url, id);
        futures.push(async move { fetch::<T>(url).await.map(|x| (id, x)) });
    }

    try_join_all(futures).await
}
