use js_sys::Promise;
use types::{DeserializeOwned, Serialize};
use wasm_bindgen_futures::future_to_promise;

use crate::{fetcher::fetch, to_js};

pub fn by_id<M: DeserializeOwned + Into<V> + ?Sized, V: Serialize>(
    base_url: String,
    id: String,
) -> Promise {
    let base_url = format!("{}/{}.msg", base_url, id);

    future_to_promise(async move {
        let model = fetch::<M>(base_url).await?;
        to_js(&model.into()).map_err(Into::into)
    })
}
