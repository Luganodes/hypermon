# hypermon
Hyperliquid Validator Monitoring tool.

The minimal all in one tool to monitor your Hyperliquid validator. Built by industry professionals.

Hypermon can:
- Expose metrics for Prometheus
- Send alerts to your Telegram group (Check TODOs at the bottom)

## Installation
Checkout [releases](https://github.com/Luganodes/hypermon/releases) or build it yourself:
```bash
# Setup your rust environment
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/Luganodes/hypermon.git
cd hypermon
cargo build --release
```

## Commands and Flags
### `start`
To start the exporter with default flags
```bash
hypermon start
```
Flags:
| Name | Default | Description |
| ----------- | ----------- | ----------- |
| `--only-telegram` | false | Only start the telegram notifications |
| `--only-metrics` | false | Only start the metrics server |
| `--tg-api-key` | NONE | If `--only-telegram` is set, this is the TG bot's API key |
| `--tg-chat-id` | NONE | If `--only-telegram` is set, this is the TG channel's ID |
| `--metrics-port` | 6969 | The port on which the metrics server should serve metrics |
| `--metrics-addr` | 0.0.0.0 | The address on which the metrics server should serve metrics |
| `--info-url` | https://api.hyperliquid-testnet.xyz/info | The Info URL to scrape metrics from. Change this to scrape Mainnet metrics |

## Metrics Served
With default flags, the following will be shown after
```bash
curl localhost:6969/metrics
```

Output format:
```
# The validator's recent blocks
hyperliquid_validator_recent_blocks{address="val address"}

# The validator's jail status
hyperliquid_validator_is_jailed{address="val address"}

# The validator's stake
hyperliquid_validator_stake{address="val address"}

# The total active stake on the network
hyperliquid_network_total_active_stake 

# The total jailed stake on the network
hyperliquid_network_total_jailed_stake 

# The total validators on the network
hyperliquid_network_total_validators 

# The time it takes to make a request to the Info endpoint
hyperliquid_request_time 
```

## Todo
- [x] Add support for telegram notifications
- [x] Add a TUI dashboard to view the network info for all validators
- [x] Fix `--only-*` flags
- [ ] Pull valuable/necessary info from EVM RPC if it is provided
- [ ] Create setup script for easy download and systemd service setup
- [ ] Show valuable info from the data directory
- [ ] When making requests to EVM, parallelize the requests
- [ ] Add a metric to show version of the node binary
