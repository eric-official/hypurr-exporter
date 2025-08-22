# hypurr-exporter

## Table of Contents

1. [Overview](#overview)  
2. [Prerequisites](#prerequisites)  
   - [System Requirements](#system-requirements)  
   - [Accounts & API Keys](#accounts--api-keys)  
   - [Configuration](#configuration)  
3. [Labels](#labels)  
4. [Metrics](#metrics)  
5. [Usage](#usage)  
   - [Local](#local)  
   - [Docker](#docker)  
6. [Local Demo](#local-demo)  


## Overview

The Hypurr exporter provides a custom Prometheus exporter to expose metrics about a user, a vault and general information on Hyperliquuid. General-purpose exporters, like the node exporter, provide relevant metrics for a broad range of companies and use cases. To answer questions about our infrastructure and individual use cases, this custom Prometheus exporter is developed.

## Prerequisites

Before running the **Hypurr Exporter**, make sure you have the following installed and configured:

### System Requirements

- **Rust toolchain** (stable, e.g. installed via [rustup](https://rustup.rs/))
- **Cargo** (comes with Rust)
- **Docker** and **Docker Compose**

### Accounts & API Keys

The exporter fetches on-chain and off-chain data from multiple providers.You will need valid API keys for the following services:

- **[Alchemy:](https://www.alchemy.com/)** Provides RPC access to Hyperliquid nodes
- **[CoinGecko:](https://www.coingecko.com/)** Provides market data and token prices

### Configuration

- Create a `config.toml` file in the project root.
- Fill in your wallet/vault addresses and the API keys.

  ```toml
  user_address = "your-wallet-address"
  vault_address = "your-vault-address"
  coingecko_key = "your-coingecko-key"
  alchemy_key = "your-alchemy-key"
  ```

## Labels

- `timestamp`: Timestamp of the exporter initialization

## Metrics

Currently, the following metrics are exposed:


| Name                                   | Type  | Description                                                                                                                        |
| ---------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `hyperliquid_price`                    | Gauge | The current market price of the Hyperliquid token (HYPE) in USD                                                                    |
| `hyperliquid_marketcap     `           | Gauge | The total market value of Hyperliquid's circulating supply                                                                         |
| `hyperliquid_fdv     `                 | Gauge | The theoretical market capitalization of a coin if the entirety of its supply is in circulation, based on its current market price |
| `hyperliquid_tvl     `                 | Gauge | Capital deposited into the platform in the form of loan collateral or liquidity trading pool                                       |
| `hyperliquid_circulating_supply     `  | Gauge | The amount of coins that are circulating in the market and are tradeable by the public                                             |
| `hyperliquid_total_supply     `        | Gauge | The amount of coins that have already been created, minus any coins that have been burned                                          |
| `hyperliquid_block_number     `        | Gauge | The current block number of the HyperEVM                                                                                           |
| `hyperliquid_base_fee     `            | Gauge | The current base fee for the next small block on HyperEVM                                                                          |
| `hyperliquid_af_account_value     `    | Gauge | The current account value of the Hyperliquid Assistance Fund                                                                       |
| `hyperliquid_num_spot_tokens     `     | Gauge | The current number of spot tokens on Hyperliquid                                                                                   |
| `hyperliquid_num_perp_tokens     `     | Gauge | The current number of perp tokens on Hyperliquid                                                                                   |
| `vault_value     `                     | Gauge | The total value locked (TVL) of the vault                                                                                          |
| `vault_apr     `                       | Gauge | The current number of perp tokens on Hyperliquid                                                                                   |
| `vault_leader_fraction     `           | Gauge | The fraction of the vault controlled or owned by the leader                                                                        |
| `vault_leader_comission     `          | Gauge | The commission that the leader earns                                                                                               |
| `vault_num_followers     `             | Gauge | The number of followers of the vault                                                                                               |
| `vault_max_distributable     `         | Gauge | The maximum amount that can be distributed from the vault                                                                          |
| `vault_max_withdrawable     `          | Gauge | The maximum amount that can be withdrawn from the vault                                                                            |
| `vault_is_closed     `                 | Gauge | A flag indicating whether the vault is closed or not                                                                               |
| `vault_allow_deposits     `            | Gauge | A flag indicating whether new deposits are allowed into the vault                                                                  |
| `user_account_value     `              | Gauge | The value of the user wallet                                                                                                       |
| `user_pnl     `                        | Gauge | The profitability of the user                                                                                                      |
| `user_staking_delegated     `          | Gauge | The value of funds delegated to stakers                                                                                            |
| `user_staking_undelegated     `        | Gauge | The value of funds undelegated from stakers                                                                                        |
| `user_staking_pending_withdrawal     ` | Gauge | The value of funds which are waiting be unstaked                                                                                   |
| `user_num_open_orders     `            | Gauge | The number of open orders by a user                                                                                                |
| `user_value_open_orders     `          | Gauge | The value of open orders by a user                                                                                                 |

## Usage

### Local

The Hypurr exporter can be used by running the following two commands in seperate terminal windows:

```
    cargo run
    curl http://0.0.0.0:3000/metrics
```

### Docker

Another option to execute the Hypurr exporter is via Docker

```
    docker build -t ghcr.io/hypurr/exporter .
    docker run -p 3000:3000 -v ./config.toml:/app/config.toml ghcr.io/hypurr/exporter
```

## Local Demo

The Hypurr exporter really shines when it’s part of a real monitoring setup. In that kind of environment, the metrics get stored in Prometheus (a time-series database) and then displayed nicely in Grafana dashboards. To get a taste of that locally, just follow the steps below:

**Step 1. Start the stack**

From the project root, run:

```
docker compose up -d
```

This will start three services:

- **hypurr-exporter:** Rust exporter serving metrics at port 3000
- **Prometheus:** scraping metrics every 60s at port 9090
- **Grafana:** visualizing metrics at port 8000

**Step 2. Access Grafana**

Open your browser at:

[http://localhost:8000](http://localhost:8000)

Login credentials:

- **Username:** admin

- **Password:** admin

**Step 3. Configure Prometheus as a data source**

In Grafana:

- Connections → Data sources → Add data source

- Select Prometheus

- Set URL to:

```
http://prometheus:9090
```

- Click Save & test — you should see “Data source is working”.

**Step 4: Have fun designing your dashboard**
