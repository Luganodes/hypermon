use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
};
use prometheus::Encoder;
use reqwest::Client;
use tracing::info;

use crate::{
    helpers::{
        escape_for_telegram_markdown_v2, get_network_validators, get_request_client, Sender,
    },
    types::HypermonError,
    Metrics,
};

pub async fn start(
    listen_addr: String,
    port: u16,
    info_url: String,
    token: String,
    chat_id: String,
) -> Result<Server, HypermonError> {
    let metrics = Metrics::new();
    metrics.register()?;

    let client = get_request_client();

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
            .app_data(web::Data::new(metrics.clone()))
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
    metrics: Data<Metrics>,
) -> Result<HttpResponse, HypermonError> {
    info!("Request to: {}", req.head().uri);

    let mut total_active_stake: f64 = 0.0;
    let mut total_jailed_stake: f64 = 0.0;

    let validators = get_network_validators(&client, info_url.into_inner().to_string()).await?;

    for validator in validators.iter() {
        let addr = validator.validator.as_str();
        let is_jailed = if validator.is_jailed { 1.0 } else { 0.0 };
        let stake = validator.stake as f64;
        let name = escape_for_telegram_markdown_v2(&validator.name.clone());

        let last_jailed = metrics.is_jailed.with_label_values(&[addr]).get();
        if !last_jailed.eq(&is_jailed) {
            if is_jailed == 1.0 {
                _ = sender
                    .send_message(format!("🚨 *{}* is now __jailed__\\!", name))
                    .await;
            } else {
                _ = sender
                    .send_message(format!("✅ *{}* is now __unjailed__\\!", name))
                    .await;
            }
        }

        let last_stake = metrics.stake.with_label_values(&[addr]).get();
        if !last_stake.eq(&stake) && last_stake != 0.0 {
            _ = sender
                .send_message(format!(
                    "🥩 *{}* stake changed by __{}__ to *{}*\\!",
                    name,
                    (stake - last_stake).to_string().replace(".", "\\."),
                    stake.to_string().replace(".", "\\.")
                ))
                .await;
        }

        metrics
            .recent_blocks
            .with_label_values(&[addr])
            .set(validator.n_recent_blocks as f64);
        metrics.is_jailed.with_label_values(&[addr]).set(is_jailed);
        metrics
            .stake
            .with_label_values(&[addr])
            .set(validator.stake as f64);

        if !validator.is_jailed {
            total_active_stake += validator.stake as f64;
        } else {
            total_jailed_stake += validator.stake as f64;
        }
    }

    let total_vals = validators.len() as f64;
    if !metrics.total_validators.get().eq(&total_vals) {
        _ = sender
            .send_message(format!(
                "\\#️⃣ Total validators on the network: __{}__\\!",
                total_vals
            ))
            .await;
    }

    metrics.total_active_stake.set(total_active_stake);
    metrics.total_jailed_stake.set(total_jailed_stake);
    metrics.total_validators.set(total_vals);

    let (encoder, buffer) = metrics.get_encoder_and_buffer()?;

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Content-Type", encoder.format_type()))
        .body(buffer))
}

async fn health_check(req: HttpRequest) -> HttpResponse {
    info!("Request to: {}", req.head().uri);
    HttpResponse::Ok().finish()
}
