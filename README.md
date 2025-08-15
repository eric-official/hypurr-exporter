# enerando-exporter

## Table of Contents

1. [Overview](#overview)
2. [Labels](#labels)
3. [Metrics](#metrics)

## Overview

The Hypurr exporter provides a custom Prometheus exporter to expose metrics about a user, a vault and general information on Hyperliquuid. General-purpose exporters, like the node exporter, provide relevant metrics for a broad range of companies and use cases. To answer questions about our infrastructure and individual use cases, this custom Prometheus exporter is developed.

## Labels

- `timestamp`: Timestamp of the exporter initialization

## Metrics

Currently, the following metrics are exposed:

| Name                               | Type  | Description                                                                 |
| ---------------------------------- | ----- | --------------------------------------------------------------------------- |
| `hyperliquid_price`           | Gauge | The current market price of the Hyperliquid token (HYPE) in USD              |
| `hyperliquid_marketcap     ` | Gauge | The total market value of Hyperliquid's circulating supply |
| `hyperliquid_fdv     ` | Gauge | The theoretical market capitalization of a coin if the entirety of its supply is in circulation, based on its current market price |
| `hyperliquid_tvl     ` | Gauge | Capital deposited into the platform in the form of loan collateral or liquidity trading pool |
| `hyperliquid_circulating_supply     ` | Gauge | The amount of coins that are circulating in the market and are tradeable by the public |
| `hyperliquid_total_supply     ` | Gauge | The amount of coins that have already been created, minus any coins that have been burned |
| `hyperliquid_block_number     ` | Gauge | The current block number of the HyperEVM |
| `hyperliquid_base_fee     ` | Gauge | The current base fee for the next small block on HyperEVM |
| `hyperliquid_af_account_value     ` | Gauge | The current account value of the Hyperliquid Assistance Fund |
| `hyperliquid_num_spot_tokens     ` | Gauge | The current number of spot tokens on Hyperliquid |
| `hyperliquid_num_perp_tokens     ` | Gauge | The current number of perp tokens on Hyperliquid |

## Usage

The Hypurr exporter can be used by running the following two commands in seperate terminal windows:

    cargo run
    curl http://0.0.0.0:3000/metrics
