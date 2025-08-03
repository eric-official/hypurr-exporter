use prometheus::{Error, Gauge, Opts, Registry};

#[derive(Debug)]
pub struct Metrics {
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
                "The profitability of the user",
            ))?,
            user_pnl: Gauge::with_opts(Opts::new("user_pnl", "The profitability of the vault"))?,
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
        vault_details: (f64, f64, f64, f64, f64, usize, f64, f64, bool, bool),
        user_details: (f64, f64, f64, f64, f64, usize, f64),
    ) -> Result<(), Error> {
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
