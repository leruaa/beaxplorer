use bytes::Buf;
use types::models::EpochModel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn get_epoch(epoch: String) -> Result<JsValue, JsValue> {
    let response = reqwest::get(format!("http://localhost:3000/data/epochs/{}.msg", epoch)).await;

    match response {
        Ok(response) => {
            let epoch =
                rmp_serde::from_read::<_, EpochModel>(response.bytes().await.unwrap().reader());

            match epoch {
                Ok(epoch) => {
                    let value = JsValue::from_serde(&epoch);

                    match value {
                        Err(_) => Err(JsValue::from_str("from_serde")),
                        Ok(value) => Ok(value),
                    }
                }
                Err(_) => Err(JsValue::from_str("from_read")),
            }
        }
        Err(err) => Err(JsValue::from_str(&format!("request: {}", err))),
    }
}
