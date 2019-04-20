#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::error::Error;

#[cfg(feature = "native")]
use reqwest::{get, Url};

#[cfg(feature = "native")]
pub fn request(url: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(url.as_ref())?;
    let mut res = get(url)?;
    let text = res.text()?;
    Ok(text)
}

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::request;
