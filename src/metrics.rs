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
}

impl Metrics {
    pub fn new() -> Result<Self, Error> {
        let metrics = Metrics {
            vault_value: Gauge::with_opts(Opts::new(
                "vault_value",
                "Charging total value locked (TVL) of the vault",
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

        Ok(())
    }

    pub fn update(
        &self,
        vault_value: f64,
        vault_pnl: f64,
        vault_apr: f64,
        vault_leader_fraction: f64,
        vault_leader_comission: f64,
        vault_num_followers: usize,
        vault_max_distributable: f64,
        vault_max_withdrawable: f64,
        vault_is_closed: bool,
        vault_allow_deposits: bool,
    ) -> Result<(), Error> {
        self.vault_value.set(vault_value);
        self.vault_pnl.set(vault_pnl);
        self.vault_apr.set(vault_apr);
        self.vault_leader_fraction.set(vault_leader_fraction);
        self.vault_leader_comission.set(vault_leader_comission);
        self.vault_num_followers.set(vault_num_followers as f64);
        self.vault_max_distributable.set(vault_max_distributable);
        self.vault_max_withdrawable.set(vault_max_withdrawable);
        self.vault_is_closed.set(bool_to_f64(vault_is_closed));
        self.vault_is_closed.set(bool_to_f64(vault_allow_deposits));

        Ok(())
    }
}

fn bool_to_f64(v: bool) -> f64 {
    match v {
        true => 1_f64,
        false => 0_f64,
    }
}
