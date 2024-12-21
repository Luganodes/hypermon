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
    rpc::RpcClient,
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
    let rpc_client = RpcClient::new(rpc_url)?;

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
            .app_data(web::Data::new(rpc_client.clone()))
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
    rpc_client: Data<RpcClient>,
    info_url: Data<String>,
    sender: Data<Sender>,
    metrics: Data<Metrics>,
) -> Result<HttpResponse, HypermonError> {
    info!("Request to: {}", req.head().uri);

    let rpc_url = rpc_client.rpc_url.clone();
    let validators =
        get_network_validators(&client, info_url.clone().into_inner().to_string()).await?;

    metrics.update_for_validators(validators, sender).await?;
    metrics.update_for_rpc(&rpc_client.into_inner()).await?;

    let (encoder, mut buffer) = metrics.get_encoder_and_buffer()?;
    let info_url_metric = format!("# HELP hyperliquid_info_url The Hyperliquid Info URL being used\n# TYPE hyperliquid_info_url gauge\nhyperliquid_info_url{{url=\"{}\"}} 1", info_url.to_string()).into_bytes();
    let rpc_url_metric = format!("\n# HELP hyperliquid_rpc_url The Hyperliquid RPC URL being used\n# TYPE hyperliquid_rpc_url gauge\nhyperliquid_rpc_url{{url=\"{}\"}} 1\n",rpc_url).into_bytes();
    
    buffer.extend(&info_url_metric);
    buffer.extend(&rpc_url_metric);

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Content-Type", encoder.format_type()))
        .body(buffer))
}

async fn health_check(req: HttpRequest) -> HttpResponse {
    info!("Request to: {}", req.head().uri);
    HttpResponse::Ok().finish()
}
