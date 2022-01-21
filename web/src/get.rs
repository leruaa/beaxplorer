use js_sys::Promise;
use types::{DeserializeOwned, Serialize};
use wasm_bindgen_futures::future_to_promise;

use crate::{fetcher::fetch, to_js};

pub fn by_id<V: DeserializeOwned + Serialize + ?Sized>(base_url: String, id: String) -> Promise {
    let base_url = format!("{}/{}.msg", base_url, id);

    future_to_promise(async move {
        let view = fetch::<V>(base_url).await?;
        to_js(&view).map_err(Into::into)
    })
}
