use clap::ArgMatches;
use tracing::info;

use crate::server;

pub async fn start(args: &ArgMatches) -> anyhow::Result<()> {
    let only_metrics = args.get_one::<bool>("only-metrics").copied().unwrap();
    let only_telegram = args.get_one::<bool>("only-telegram").copied().unwrap();
    let tg_api_key = args.get_one::<&str>("tg-api-key").copied();
    let tg_chat_id = args.get_one::<&str>("tg-chat-id").copied();
    let metrics_port = args.get_one::<u16>("metrics-port").copied().unwrap();
    let metrics_addr: String = args.get_one::<String>("metrics-addr").unwrap().to_string();
    let info_url = args.get_one::<String>("info-url").unwrap().to_string();

    info!("===================");
    info!("Args found: ");
    info!("--only-metrics?: {}", only_metrics);
    info!("--only-telegram?: {}", only_telegram);
    info!("--tg-api-key: {:?}", tg_api_key);
    info!("--tg-chat-id: {:?}", tg_chat_id);
    info!("--metrics-port: {}", metrics_port);
    info!("--metrics-addr: {}", metrics_addr);
    info!("--info-url: {}", info_url);
    info!("===================");

    // Start the prometheus server
    let server_handle = tokio::spawn(async move {
        _ = server::start(metrics_addr, metrics_port, info_url)
            .await
            .unwrap()
            .await;
    });

    // and optionally start the telegram bot if the API key and chat ID are present

    _ = server_handle.await;
    Ok(())
}
