use clap::{value_parser, Arg, Command};
use hypermon::commands::{show, start};
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        .with_target(true)
        .with_ansi(true)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Comes from the build script
    let version_str = include_str!(concat!(env!("OUT_DIR"), "/version_file"));

    let matches = Command::new("hypermon")
        .about("Minimal, all-in-one Hyperliquid Validator Metrics Exporter")
        .author("Suryansh @ Luganodes")
        .version(version_str)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Start monitoring")
                .arg_required_else_help(false)
                .args([
                    Arg::new("tg-api-key")
                        .long("tg-api-key")
                        .requires("tg-chat-id")
                        .default_value(""),
                    Arg::new("tg-chat-id")
                        .long("tg-chat-id")
                        .requires("tg-api-key")
                        .default_value(""),
                    Arg::new("metrics-port")
                        .long("metrics-port")
                        .value_parser(value_parser!(u16))
                        .default_value("6969"),
                    Arg::new("metrics-addr")
                        .long("metrics-addr")
                        .default_value("0.0.0.0"),
                    Arg::new("info-url")
                        .long("info-url")
                        .help("A Hyperliquid info url. Can be different for testnet and mainnet.")
                        .default_value("https://api.hyperliquid-testnet.xyz/info"),
                    Arg::new("rpc-url")
                        .long("rpc-url")
                        .help("A Hyperliquid EVM JSON RPC URL")
                        .required(true),
                ]),
        )
        .subcommand(
            Command::new("show")
                .about("Show the network's validators state")
                .arg_required_else_help(false)
                .args([
                    Arg::new("info-url")
                        .help("The info url")
                        .long("info-url")
                        .default_value("https://api.hyperliquid-testnet.xyz/info"),
                    Arg::new("filter-address")
                        .help("Show all information for only the validator address given")
                        .long("filter-address")
                        .default_value("0x1ab189b7801140900c711e458212f9c76f8dac79"),
                    Arg::new("only-jailed")
                        .value_parser(value_parser!(bool))
                        .help("Show only the jailed validators")
                        .long("only-jailed")
                        .action(clap::ArgAction::SetTrue),
                    Arg::new("only-active")
                        .value_parser(value_parser!(bool))
                        .help("Show only the active validators")
                        .long("only-active")
                        .action(clap::ArgAction::SetTrue),
                ]),
        )
        .get_matches();

    let res = match matches.subcommand() {
        Some(("start", sub_m)) => start(sub_m).await,
        Some(("show", sub_m)) => show(sub_m).await,
        None | Some(_) => unreachable!(),
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            error!("Error: {err:?}");
        }
    }

    Ok(())
}
