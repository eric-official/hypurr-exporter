use prometheus::{Error, Gauge, Opts, Registry};

#[derive(Debug)]
pub struct Metrics {
    pub hyperliquid_price: Gauge,
    pub hyperliquid_marketcap: Gauge,
    pub hyperliquid_fdv: Gauge,
    pub hyperliquid_tvl: Gauge,
    pub hyperliquid_circulating_supply: Gauge,
    pub hyperliquid_total_supply: Gauge,

    pub hyperliquid_block_number: Gauge,
    pub hyperliquid_base_fee: Gauge,
    pub hyperliquid_af_account_value: Gauge,
    pub hyperliquid_num_spot_tokens: Gauge,
    pub hyperliquid_num_perp_tokens: Gauge,

    pub vault_value: Gauge,
    pub vault_pnl: Gauge,
    pub vault_apr: Gauge,
    pub vault_leader_fraction: Gauge,
    pub vault_leader_comission: Gauge,
    pub vault_num_followers: Gauge,
    pub vault_max_distributable: Gauge,
    pub vault_max_withdrawable: Gauge,
    pub vault_is_closed: Gauge,
    pub vault_allow_deposits: Gauge,

    pub user_account_value: Gauge,
    pub user_pnl: Gauge,
    pub user_staking_delegated: Gauge,
    pub user_staking_undelegated: Gauge,
    pub user_staking_pending_withdrawal: Gauge,
    pub user_num_open_orders: Gauge,
    pub user_value_open_orders: Gauge,
}

impl Metrics {
    pub fn new() -> Result<Self, Error> {
        let metrics = Metrics {
            hyperliquid_price: Gauge::with_opts(Opts::new(
                "hyperliquid_price",
                "The current market price of the Hyperliquid token (HYPE) in USD",
            ))?,
            hyperliquid_marketcap: Gauge::with_opts(Opts::new(
                "hyperliquid_marketcap",
                "The total market value of Hyperliquid's circulating supply",
            ))?,
            hyperliquid_fdv: Gauge::with_opts(Opts::new(
                "hyperliquid_fdv",
                "The theoretical market capitalization of a coin if the entirety of its supply is in circulation, based on its current market price",
            ))?,
            hyperliquid_tvl: Gauge::with_opts(Opts::new(
                "hyperliquid_tvl",
                "Capital deposited into the platform in the form of loan collateral or liquidity trading pool",
            ))?,
            hyperliquid_circulating_supply: Gauge::with_opts(Opts::new(
                "hyperliquid_circulating_supply",
                "The amount of coins that are circulating in the market and are tradeable by the public",
            ))?,
            hyperliquid_total_supply: Gauge::with_opts(Opts::new(
                "hyperliquid_total_supply",
                "The amount of coins that have already been created, minus any coins that have been burned",
            ))?,

            hyperliquid_block_number: Gauge::with_opts(Opts::new(
                "hyperliquid_block_number",
                "The current block number of the HyperEVM",
            ))?,
            hyperliquid_base_fee: Gauge::with_opts(Opts::new(
                "hyperliquid_base_fee",
                "The current base fee for the next small block on HyperEVM",
            ))?,
            hyperliquid_af_account_value: Gauge::with_opts(Opts::new(
                "hyperliquid_af_account_value",
                "The current account value of theHyperliquid Assistance Fund",
            ))?,
            hyperliquid_num_spot_tokens: Gauge::with_opts(Opts::new(
                "hyperliquid_num_spot_tokens",
                "The current number of spot tokens on Hyperliquid",
            ))?,
            hyperliquid_num_perp_tokens: Gauge::with_opts(Opts::new(
                "hyperliquid_num_perp_tokens",
                "TThe current number of perp tokens on Hyperliquid",
            ))?,


            vault_value: Gauge::with_opts(Opts::new(
                "vault_value",
                "The total value locked (TVL) of the vault",
            ))?,
            vault_pnl: Gauge::with_opts(Opts::new("vault_pnl", "The profitability of the vault"))?,
            vault_apr: Gauge::with_opts(Opts::new(
                "vault_apr",
                "The annual percentage rate (APR) for the vault",
            ))?,
            vault_leader_fraction: Gauge::with_opts(Opts::new(
                "vault_leader_fraction",
                "The fraction of the vault controlled or owned by the leader",
            ))?,
            vault_leader_comission: Gauge::with_opts(Opts::new(
                "vault_leader_comission",
                "The commission that the leader earns",
            ))?,
            vault_num_followers: Gauge::with_opts(Opts::new(
                "vault_num_followers",
                "The number of followers of the vault",
            ))?,
            vault_max_distributable: Gauge::with_opts(Opts::new(
                "vault_max_distributable",
                "The maximum amount that can be distributed from the vault",
            ))?,
            vault_max_withdrawable: Gauge::with_opts(Opts::new(
                "vault_max_withdrawable",
                "The maximum amount that can be withdrawn from the vault",
            ))?,
            vault_is_closed: Gauge::with_opts(Opts::new(
                "vault_is_closed",
                "A flag indicating whether the vault is closed or not",
            ))?,
            vault_allow_deposits: Gauge::with_opts(Opts::new(
                "vault_allow_deposits",
                "A flag indicating whether new deposits are allowed into the vault",
            ))?,

            user_account_value: Gauge::with_opts(Opts::new(
                "user_account_value",
                "The value of the user wallet",
            ))?,
            user_pnl: Gauge::with_opts(Opts::new("user_pnl", "The profitability of the user"))?,
            user_staking_delegated: Gauge::with_opts(Opts::new(
                "user_staking_delegated",
                "The value of funds delegated to stakers",
            ))?,
            user_staking_undelegated: Gauge::with_opts(Opts::new(
                "user_staking_undelegated",
                "The value of funds undelegated from stakers",
            ))?,
            user_staking_pending_withdrawal: Gauge::with_opts(Opts::new(
                "user_staking_pending_withdrawal",
                "The value of funds which are waiting be unstaked",
            ))?,
            user_num_open_orders: Gauge::with_opts(Opts::new(
                "user_num_open_orders",
                "The number of open orders by a user",
            ))?,
            user_value_open_orders: Gauge::with_opts(Opts::new(
                "user_value_open_orders",
                "The value of open orders by a user",
            ))?,
        };

        Ok(metrics)
    }

    pub fn register(&self, registry: &Registry) -> Result<(), Error> {
        registry.register(Box::new(self.hyperliquid_price.clone()))?;
        registry.register(Box::new(self.hyperliquid_marketcap.clone()))?;
        registry.register(Box::new(self.hyperliquid_fdv.clone()))?;
        registry.register(Box::new(self.hyperliquid_tvl.clone()))?;
        registry.register(Box::new(self.hyperliquid_circulating_supply.clone()))?;
        registry.register(Box::new(self.hyperliquid_total_supply.clone()))?;

        registry.register(Box::new(self.hyperliquid_block_number.clone()))?;
        registry.register(Box::new(self.hyperliquid_base_fee.clone()))?;
        registry.register(Box::new(self.hyperliquid_af_account_value.clone()))?;
        registry.register(Box::new(self.hyperliquid_num_spot_tokens.clone()))?;
        registry.register(Box::new(self.hyperliquid_num_perp_tokens.clone()))?;

        registry.register(Box::new(self.vault_value.clone()))?;
        registry.register(Box::new(self.vault_pnl.clone()))?;
        registry.register(Box::new(self.vault_apr.clone()))?;
        registry.register(Box::new(self.vault_leader_fraction.clone()))?;
        registry.register(Box::new(self.vault_leader_comission.clone()))?;
        registry.register(Box::new(self.vault_num_followers.clone()))?;
        registry.register(Box::new(self.vault_max_distributable.clone()))?;
        registry.register(Box::new(self.vault_max_withdrawable.clone()))?;
        registry.register(Box::new(self.vault_is_closed.clone()))?;
        registry.register(Box::new(self.vault_allow_deposits.clone()))?;

        registry.register(Box::new(self.user_account_value.clone()))?;
        registry.register(Box::new(self.user_pnl.clone()))?;
        registry.register(Box::new(self.user_staking_delegated.clone()))?;
        registry.register(Box::new(self.user_staking_undelegated.clone()))?;
        registry.register(Box::new(self.user_staking_pending_withdrawal.clone()))?;
        registry.register(Box::new(self.user_num_open_orders.clone()))?;
        registry.register(Box::new(self.user_value_open_orders.clone()))?;

        Ok(())
    }

    pub fn update(
        &self,
        coingecko_financial_meta: (f64, i64, i64, i64, f64, f64),
        protocol_meta: (u64, u64, f64, usize, usize),
        vault_details: (f64, f64, f64, f64, f64, usize, f64, f64, bool, bool),
        user_details: (f64, f64, f64, f64, f64, usize, f64),
    ) -> Result<(), Error> {
        self.hyperliquid_price.set(coingecko_financial_meta.0);
        self.hyperliquid_marketcap
            .set(coingecko_financial_meta.1 as f64);
        self.hyperliquid_fdv.set(coingecko_financial_meta.2 as f64);
        self.hyperliquid_tvl.set(coingecko_financial_meta.3 as f64);
        self.hyperliquid_circulating_supply
            .set(coingecko_financial_meta.4);
        self.hyperliquid_total_supply
            .set(coingecko_financial_meta.5);

        self.hyperliquid_block_number.set(protocol_meta.0 as f64);
        self.hyperliquid_base_fee.set(protocol_meta.1 as f64);
        self.hyperliquid_af_account_value.set(protocol_meta.2);
        self.hyperliquid_num_spot_tokens.set(protocol_meta.3 as f64);
        self.hyperliquid_num_perp_tokens.set(protocol_meta.4 as f64);

        self.vault_value.set(vault_details.0);
        self.vault_pnl.set(vault_details.1);
        self.vault_apr.set(vault_details.2);
        self.vault_leader_fraction.set(vault_details.3);
        self.vault_leader_comission.set(vault_details.4);
        self.vault_num_followers.set(vault_details.5 as f64);
        self.vault_max_distributable.set(vault_details.6);
        self.vault_max_withdrawable.set(vault_details.7);
        self.vault_is_closed.set(bool_to_f64(vault_details.8));
        self.vault_is_closed.set(bool_to_f64(vault_details.9));

        self.user_account_value.set(user_details.0);
        self.user_pnl.set(user_details.1);
        self.user_staking_delegated.set(user_details.2);
        self.user_staking_undelegated.set(user_details.3);
        self.user_staking_pending_withdrawal.set(user_details.4);
        self.user_num_open_orders.set(user_details.5 as f64);
        self.user_value_open_orders.set(user_details.6);

        Ok(())
    }
}

fn bool_to_f64(v: bool) -> f64 {
    match v {
        true => 1_f64,
        false => 0_f64,
    }
}
