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
    helpers::{get_network_validators, get_request_client, Sender},
    types::HypermonError,
    Metrics,
};

pub async fn start(
    listen_addr: String,
    port: u16,
    info_url: String,
    rpc_url: String,
    token: String,
    chat_id: String,
) -> Result<Server, HypermonError> {
    let metrics = Metrics::new();
    metrics.register()?;

    let client = get_request_client();

    // If token and chat_id are not provided the sender won't be able to send anyway
    let sender = Sender { token, chat_id };

    _ = sender
        .send_message("▶️ Starting Hypermon\\!".to_string())
        .await;

    info!("▶️ Starting Hypermon!");

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

    let validators = get_network_validators(&client, info_url.into_inner().to_string()).await?;

    metrics.update_for_validators(validators, sender).await?;

    let (encoder, buffer) = metrics.get_encoder_and_buffer()?;

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Content-Type", encoder.format_type()))
        .body(buffer))
}

async fn health_check(req: HttpRequest) -> HttpResponse {
    info!("Request to: {}", req.head().uri);
    HttpResponse::Ok().finish()
}
