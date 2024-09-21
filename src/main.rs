use clap::{value_parser, Arg, Command};
use hypermon::commands::start;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_ansi(true)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let matches = Command::new("hypermon")
        .about("Minimal, all-in-one Hyperliquid Validator Metrics Exporter")
        .author("Suryansh @ Luganodes")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Start monitoring")
                .arg_required_else_help(false)
                .args([
                    Arg::new("only-telegram")
                        .value_name("BOOL")
                        .long("only-telegram")
                        .value_parser(value_parser!(bool))
                        .num_args(0..=1)
                        .default_value("false")
                        .default_missing_value("true")
                        .requires("tg-api-key")
                        .requires("tg-chat-id"),
                    Arg::new("only-metrics")
                        .value_name("BOOL")
                        .long("only-metrics")
                        .value_parser(value_parser!(bool))
                        .num_args(0..=1)
                        .default_value("false")
                        .default_missing_value("true"),
                    Arg::new("tg-api-key").long("tg-api-key").requires("tg-chat-id"),
                    Arg::new("tg-chat-id").long("tg-chat-id").requires("tg-api-key"),
                    Arg::new("metrics-port")
                        .long("metrics-port")
                        .value_parser(value_parser!(u16))
                        .default_value("6969"),
                    Arg::new("metrics-addr")
                        .long("metrics-addr")
                        .default_value("0.0.0.0"),
                    Arg::new("info-url")
                        .long("info-url")
                        .default_value("https://api.hyperliquid-testnet.xyz/info"),
                ]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_m)) => {
            let _ = start(sub_m).await;
        }
        None | Some(_) => unreachable!(),
    }

    Ok(())
}
