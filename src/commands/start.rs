use clap::ArgMatches;
use tracing::{error, info};

use crate::{server, types::HypermonError};

pub async fn start(args: &ArgMatches) -> Result<(), HypermonError> {
    let tg_api_key = args
        .get_one::<String>("tg-api-key")
        .map(|s| s.clone())
        .unwrap();
    let tg_chat_id = args
        .get_one::<String>("tg-chat-id")
        .map(|s| s.clone())
        .clone()
        .unwrap();
    let metrics_port = args.get_one::<u16>("metrics-port").copied().unwrap();
    let metrics_addr = args.get_one::<String>("metrics-addr").unwrap().to_string();
    let info_url = args.get_one::<String>("info-url").unwrap().to_string();
    let rpc_url = args.get_one::<String>("rpc-url").unwrap().to_string();

    info!("===================");
    info!("Args found: ");
    info!("--tg-api-key: {:?}", tg_api_key);
    info!("--tg-chat-id: {:?}", tg_chat_id);
    info!("--metrics-port: {}", metrics_port);
    info!("--metrics-addr: {}", metrics_addr);
    info!("--info-url: {}", info_url);
    info!("--rpc-url: {}", rpc_url);
    info!("===================");

    // Start the prometheus server
    let server_handle = tokio::spawn(async move {
        match server::start(
            metrics_addr,
            metrics_port,
            info_url,
            rpc_url,
            tg_api_key.to_string(),
            tg_chat_id.to_string(),
        )
        .await
        {
            Ok(server) => {
                _ = server.await;
            }
            Err(err) => {
                error!("Received error: {err:?}");
            }
        }
    });

    // TODO: optionally start the telegram bot if the API key and chat ID are present

    _ = server_handle.await;
    Ok(())
}
