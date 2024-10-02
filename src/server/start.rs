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

use crate::{
    helpers::Sender,
    types::{HypermonError, Query, Validator},
};

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

pub async fn start(
    listen_addr: String,
    port: u16,
    info_url: String,
    token: String,
    chat_id: String,
) -> Result<Server, HypermonError> {
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

    let sender = Sender { token, chat_id };

    _ = sender
        .send_message("▶️ Starting Hypermon\\!".to_string())
        .await;

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/metrics", web::get().to(get_metrics))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(info_url.clone()))
            .app_data(web::Data::new(sender.clone()))
    })
    .bind((listen_addr, port))?
    .run();

    Ok(server)
}

async fn get_metrics(
    req: HttpRequest,
    client: Data<Client>,
    info_url: Data<String>,
    sender: Data<Sender>,
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
        let addr = validator.validator.as_str();
        let is_jailed = if validator.is_jailed { 1.0 } else { 0.0 };
        let stake = validator.stake as f64;
        let name = validator.name.clone();

        RECENT_BLOCKS
            .with_label_values(&[addr])
            .set(validator.n_recent_blocks as f64);
        IS_JAILED.with_label_values(&[addr]).set(is_jailed);
        STAKE.with_label_values(&[addr]).set(validator.stake as f64);

        if !IS_JAILED.with_label_values(&[addr]).get().eq(&is_jailed) {
            if is_jailed == 1.0 {
                _ = sender
                    .send_message(format!("🚨 __{}__ is now *jailed\\!*", name))
                    .await;
            } else {
                _ = sender
                    .send_message(format!("✅ __{}__ is now unjailed\\!", name))
                    .await;
            }
        }

        let last_stake = STAKE.with_label_values(&[addr]).get();
        if !STAKE.with_label_values(&[addr]).get().eq(&stake) {
            _ = sender
                .send_message(format!(
                    "__{}__ stake changed by __{}__ to *{}*\\!",
                    name,
                    (stake - last_stake).to_string().replace(".", "\\."),
                    stake.to_string().replace(".", "\\.")
                ))
                .await;
        }

        if !validator.is_jailed {
            total_active_stake += validator.stake as f64;
        } else {
            total_jailed_stake += validator.stake as f64;
        }
    }

    let last_total_active_stake = TOTAL_ACTIVE_STAKE.get();
    if last_total_active_stake != total_active_stake {
        _ = sender
            .send_message(format!(
                "🥩 Total *active* network stake has changed to {}\\!",
                total_active_stake
            ))
            .await;
    }

    let last_total_jailed_stake = TOTAL_JAILED_STAKE.get();
    if last_total_jailed_stake != total_jailed_stake {
        _ = sender
            .send_message(format!(
                "🥩 Total *jailed* network stake has changed to {}\\!",
                total_jailed_stake
            ))
            .await;
    }

    let total_vals = validators.len() as f64;
    if !TOTAL_VALIDATORS.get().eq(&total_vals) {
        _ = sender
            .send_message(format!(
                "\\#️⃣ Total validators on the network: __{}__\\!",
                total_vals
            ))
            .await;
    }

    TOTAL_ACTIVE_STAKE.set(total_active_stake);
    TOTAL_JAILED_STAKE.set(total_jailed_stake);
    TOTAL_VALIDATORS.set(total_vals);

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
