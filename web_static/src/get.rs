use js_sys::{Error, Promise};
use types::{DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

use crate::fetcher::fetch;

pub fn get<V: DeserializeOwned + Serialize>(base_url: String, id: String) -> Promise {
    let base_url = format!("{}/{}.msg", base_url, id);

    future_to_promise(async move {
        let result = fetch::<V>(base_url).await;

        match result {
            Ok(result) => {
                let t = JsValue::from_serde(&result);
                match t {
                    Ok(t) => Ok(t),
                    Err(err) => Err(Error::new(&err.to_string()).into()),
                }
            }
            Err(err) => Err(Error::new(&err.to_string()).into()),
        }
    })
}
