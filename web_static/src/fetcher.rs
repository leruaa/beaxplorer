use bytes::Buf;
use types::DeserializeOwned;

use crate::DeserializeError;

pub async fn fetch<T: DeserializeOwned>(url: String) -> Result<T, DeserializeError> {
    let response = reqwest::get(url).await?;

    rmp_serde::from_read::<_, T>(response.bytes().await?.reader()).map_err(Into::into)
}
