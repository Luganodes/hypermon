use anyhow::Context;
use prometheus::{opts, Encoder, Gauge, GaugeVec, Registry, TextEncoder};

use crate::types::HypermonError;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub recent_blocks: GaugeVec,
    pub is_jailed: GaugeVec,
    pub stake: GaugeVec,
    pub total_active_stake: Gauge,
    pub total_jailed_stake: Gauge,
    pub total_validators: Gauge,
    pub request_time: Gauge,
    registry: Registry,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            recent_blocks: GaugeVec::new(
                opts!(
                    "hyperliquid_validator_recent_blocks",
                    "Recent blocks produced"
                ),
                &["address"],
            )
            .unwrap(),
            is_jailed: GaugeVec::new(
                opts!("hyperliquid_validator_is_jailed", "Is a validator jailed?"),
                &["address"],
            )
            .unwrap(),
            stake: GaugeVec::new(
                opts!("hyperliquid_validator_stake", "Stake of a validator"),
                &["address"],
            )
            .unwrap(),
            total_active_stake: Gauge::new(
                "hyperliquid_network_total_active_stake",
                "Active stake of the whole network",
            )
            .unwrap(),
            total_jailed_stake: Gauge::new(
                "hyperliquid_network_total_jailed_stake",
                "Jailed stake of the whole network",
            )
            .unwrap(),
            total_validators: Gauge::new(
                "hyperliquid_network_total_validators",
                "Total amount of validators on the network",
            )
            .unwrap(),
            request_time: Gauge::new(
                "hyperliquid_request_time",
                "The time it takes to get a response from the info endpoint",
            )
            .unwrap(),
            registry: Registry::new(),
        }
    }

    pub fn register(&self) -> Result<(), HypermonError> {
        self.registry
            .register(Box::new(self.recent_blocks.clone()))
            .context("Couldn't register recent_blocks")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.is_jailed.clone()))
            .context("Couldn't register is_jailed")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.stake.clone()))
            .context("Couldn't register stake")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.total_active_stake.clone()))
            .context("Couldn't register total_active_stake")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.total_jailed_stake.clone()))
            .context("Couldn't register total_active_stake")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.total_validators.clone()))
            .context("Couldn't register total_validators")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        self.registry
            .register(Box::new(self.request_time.clone()))
            .context("Couldn't register request_time")
            .map_err(|e| HypermonError::RegisterError(e.into()))?;
        Ok(())
    }

    pub fn get_encoder_and_buffer(&self) -> Result<(TextEncoder, Vec<u8>), HypermonError> {
        let encoder = TextEncoder::new();

        let metric_families = self.registry.gather();
        let mut buffer = vec![];
        encoder
            .encode(&metric_families, &mut buffer)
            .context("Couldn't encode metric families")
            .map_err(|e| HypermonError::EncodeError(e.into()))?;

        Ok((encoder, buffer))
    }
}
