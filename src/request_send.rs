use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error;

fn build_headers(headers: &[(String, String)]) -> Result<HeaderMap, Box<dyn Error>> {
    let mut header_map = HeaderMap::new();

    for (key, value) in headers {
        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            continue;
        }

        let header_name = HeaderName::from_bytes(key.as_bytes())?;
        let header_value = HeaderValue::from_str(value)?;

        header_map.insert(header_name, header_value);
    }

    Ok(header_map)
}

pub fn send_get(url: &str, headers: &[(String, String)]) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let header_map = build_headers(headers)?;

    let resp = client
        .get(url)
        .headers(header_map)
        .send()?
        .text()?;

    Ok(resp)
}

pub fn send_post(
    url: &str,
    body: &str,
    headers: &[(String, String)],
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let header_map = build_headers(headers)?;

    let resp = client
        .post(url)
        .headers(header_map)
        .body(body.to_string())
        .send()?
        .text()?;

    Ok(resp)
}