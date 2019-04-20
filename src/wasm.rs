use std::error::Error;

use futures::Future;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};

#[derive(Debug)]
struct ReqError {
    description: String,
}

use std::fmt;

impl fmt::Display for ReqError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description)
    }
}

impl Error for ReqError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Into<ReqError> for JsValue {
    fn into(self) -> ReqError {
        ReqError {
            description: "error".to_string(),
        }
    }
}

pub fn request(url: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(url.as_ref(), &opts).map_err(|e| {
        let err: ReqError = e.into();
        err
    })?;
    let win = window().ok_or_else(|| "Error")?;

    let promise = win.fetch_with_request(&req);
    let future = JsFuture::from(promise);
    let value = future
        .wait()
        .map_err(|e| e.as_string().unwrap_or_else(|| "Error".to_string()))?;
    let resp: Response = value.dyn_into().map_err(|e| {
        let err: ReqError = e.into();
        err
    })?;
    let promise = resp.text().map_err(|e| {
        let err: ReqError = e.into();
        err
    })?;
    let future = JsFuture::from(promise);
    let value = future
        .wait()
        .map(|val| val.as_string().unwrap_or_default())
        .map_err(|e| e.as_string().unwrap_or_else(|| "Error".to_string()))?;
    Ok(value)
}
