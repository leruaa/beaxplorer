use js_sys::Promise;
use types::{DeserializeOwned, Serialize};
use wasm_bindgen_futures::future_to_promise;

use crate::{fetcher::fetch, to_js};

pub fn by_id<M, V>(base_url: String, id: u64) -> Promise
where
    M: DeserializeOwned + ?Sized,
    V: Serialize,
    (u64, M): Into<V>,
{
    let base_url = format!("{}/{}.msg", base_url, id);

    future_to_promise(async move {
        let model = fetch::<M>(base_url).await?;
        to_js(&(id, model).into()).map_err(Into::into)
    })
}
