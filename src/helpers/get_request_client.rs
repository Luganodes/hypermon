use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};

pub fn get_request_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").expect("hello ser, json pls"),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Couldn't get client")
}
