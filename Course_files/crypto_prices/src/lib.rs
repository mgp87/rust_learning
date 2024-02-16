use sha256::digest;

pub fn get_hash(word: String) -> String {
    digest(word)
}

use serde::Deserialize;
use serde_json::{Map, Value};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn get_coin_price(coin: String) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.coingecko.com/api/v3/coins/{coin}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    let coin_data = parse_body(&text);

    if coin_data.market_data.current_price.contains_key("usd") {
        Ok(coin_data.market_data.current_price["usd"].to_string())
    } else {
        Ok("error".to_string())
    }
}

fn parse_body(body: &str) -> CoinData {
    serde_json::from_str(body).unwrap()
}

#[derive(Deserialize)]
struct CoinData {
    _id: String,
    _name: String,
    _symbol: String,
    _image: Map<String, Value>,
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    current_price: Map<String, Value>,
}
