#![deny(clippy::all)]

use curl::easy::Easy;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn http_get(url: String) -> Result<String> {
    let mut easy = Easy::new();
    easy.url(&url).map_err(|e| Error::from_reason(e.to_string()))?;

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .map_err(|e| Error::from_reason(e.to_string()))?;

        transfer.perform().map_err(|e| Error::from_reason(e.to_string()))?;
    }

    Ok(String::from_utf8_lossy(&data).to_string())
}