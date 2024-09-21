use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Context;
use lazy_static::lazy_static;
use prometheus::{opts, Counter, IntCounter, IntCounterVec};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};
use tracing::info;

use crate::types::{HypermonError, Query, Validator};

lazy_static! {
    static ref RECENT_BLOCKS: IntCounterVec = IntCounterVec::new(
        opts!(
            "hyperliquid_validator_recent_blocks",
            "Recent blocks produced"
        ),
        &["address"]
    )
    .unwrap();
    static ref IS_JAILED: IntCounterVec = IntCounterVec::new(
        opts!("hyperliquid_validator_is_jailed", "Is a validator jailed?"),
        &["address"]
    )
    .unwrap();
    static ref STAKE: IntCounterVec = IntCounterVec::new(
        opts!("hyperliquid_validator_stake", "Stake of a validator"),
        &["address"]
    )
    .unwrap();
    static ref TOTAL_ACTIVE_STAKE: IntCounter = IntCounter::new(
        "hyperliquid_network_total_active_stake",
        "Active stake of the whole network"
    )
    .unwrap();
    static ref TOTAL_JAILED_STAKE: IntCounter = IntCounter::new(
        "hyperliquid_network_total_jailed_stake",
        "Jailed stake of the whole network"
    )
    .unwrap();
    static ref TOTAL_VALIDATORS: IntCounter = IntCounter::new(
        "hyperliquid_network_total_validators",
        "Total amount of validators on the network"
    )
    .unwrap();
    static ref REQUEST_TIME: Counter = Counter::new(
        "hyperliquid_request_time",
        "The time it takes to get a response from the info endpoint"
    )
    .unwrap();
}

pub async fn start(listen_addr: String, port: u16, info_url: String) -> std::io::Result<Server> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").expect("hello ser, json pls"),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Couldn't get client");

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/metrics", web::get().to(get_metrics))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(info_url.clone()))
    })
    .bind((listen_addr, port))?
    .run();

    Ok(server)
}

async fn get_metrics(
    client: Data<Client>,
    info_url: Data<String>,
) -> Result<HttpResponse, HypermonError> {
    info!("Sending metrics...");

    let validators = client
        .post(info_url.clone().into_inner().to_string())
        .json(&Query {
            t: "validatorSummaries".to_string(),
        })
        .send()
        .await
        .context(format!(
            "Error with the response from: {}",
            info_url.into_inner()
        ))
        .map_err(|e| HypermonError::ResponseError(e))?
        .json::<Vec<Validator>>()
        .await
        .context("Error while deserializing Validator summaries")
        .map_err(|e| HypermonError::DeserializationError(e))?;

    for validator in validators {
        info!("{}", validator);
    }

    Ok(HttpResponse::Ok().finish())
}

async fn health_check() -> HttpResponse {
    info!("something");
    HttpResponse::Ok().finish()
}
