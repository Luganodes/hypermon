use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
};
use anyhow::Context;
use lazy_static::lazy_static;
use prometheus::{opts, Encoder, Gauge, GaugeVec, Registry, TextEncoder};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};
use tracing::{error, info};

use crate::types::{HypermonError, Query, Validator};

lazy_static! {
    static ref RECENT_BLOCKS: GaugeVec = GaugeVec::new(
        opts!(
            "hyperliquid_validator_recent_blocks",
            "Recent blocks produced"
        ),
        &["address"]
    )
    .unwrap();
    static ref IS_JAILED: GaugeVec = GaugeVec::new(
        opts!("hyperliquid_validator_is_jailed", "Is a validator jailed?"),
        &["address"]
    )
    .unwrap();
    static ref STAKE: GaugeVec = GaugeVec::new(
        opts!("hyperliquid_validator_stake", "Stake of a validator"),
        &["address"]
    )
    .unwrap();
    static ref TOTAL_ACTIVE_STAKE: Gauge = Gauge::new(
        "hyperliquid_network_total_active_stake",
        "Active stake of the whole network"
    )
    .unwrap();
    static ref TOTAL_JAILED_STAKE: Gauge = Gauge::new(
        "hyperliquid_network_total_jailed_stake",
        "Jailed stake of the whole network"
    )
    .unwrap();
    static ref TOTAL_VALIDATORS: Gauge = Gauge::new(
        "hyperliquid_network_total_validators",
        "Total amount of validators on the network"
    )
    .unwrap();
    static ref REQUEST_TIME: Gauge = Gauge::new(
        "hyperliquid_request_time",
        "The time it takes to get a response from the info endpoint"
    )
    .unwrap();
    static ref REGISTRY: Registry = Registry::new();
}

pub async fn start(listen_addr: String, port: u16, info_url: String) -> Result<Server, HypermonError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").expect("hello ser, json pls"),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Couldn't get client");

    REGISTRY
        .register(Box::new(RECENT_BLOCKS.clone()))
        .context("Couldn't register recent_blocks")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(IS_JAILED.clone()))
        .context("Couldn't register is_jailed")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(STAKE.clone()))
        .context("Couldn't register stake")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(TOTAL_ACTIVE_STAKE.clone()))
        .context("Couldn't register total_active_stake")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(TOTAL_JAILED_STAKE.clone()))
        .context("Couldn't register total_active_stake")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(TOTAL_VALIDATORS.clone()))
        .context("Couldn't register total_validators")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;
    REGISTRY
        .register(Box::new(REQUEST_TIME.clone()))
        .context("Couldn't register request_time")
        .map_err(|e| HypermonError::RegisterError(e.into()))?;


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
    req: HttpRequest,
    client: Data<Client>,
    info_url: Data<String>,
) -> Result<HttpResponse, HypermonError> {
    info!("Request to: {}", req.head().uri);

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
        .map_err(|e| {
            error!("{e:?}");
            HypermonError::DeserializationError(e)
        })?;

    let mut total_active_stake: f64 = 0.0;
    let mut total_jailed_stake: f64 = 0.0;

    for validator in validators.iter() {
        RECENT_BLOCKS
            .with_label_values(&[validator.validator.as_str()])
            .set(validator.n_recent_blocks as f64);
        IS_JAILED
            .with_label_values(&[validator.validator.as_str()])
            .set(if validator.is_jailed { 1.0 } else { 0.0 });
        STAKE
            .with_label_values(&[validator.validator.as_str()])
            .set(validator.stake as f64);

        if !validator.is_jailed {
            total_active_stake += validator.stake as f64;
        } else {
            total_jailed_stake += validator.stake as f64;
        }
    }

    TOTAL_ACTIVE_STAKE.set(total_active_stake);
    TOTAL_JAILED_STAKE.set(total_jailed_stake);
    TOTAL_VALIDATORS.set(validators.len() as f64);

    let encoder = TextEncoder::new();

    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];
    encoder
        .encode(&metric_families, &mut buffer)
        .context("Couldn't encode metric families")
        .map_err(|e| HypermonError::EncodeError(e.into()))?;

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Content-Type", encoder.format_type()))
        .body(buffer))
}

async fn health_check(req: HttpRequest) -> HttpResponse {
    info!("Request to: {}", req.head().uri);
    HttpResponse::Ok().finish()
}
