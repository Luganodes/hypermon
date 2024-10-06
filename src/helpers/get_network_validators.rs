use anyhow::Context;
use reqwest::Client;
use tracing::error;

use crate::types::{HypermonError, Query, Validator};

pub async fn get_network_validators(
    client: &Client,
    info_url: String,
) -> Result<Vec<Validator>, HypermonError> {
    let mut validators = client
        .post(info_url.clone())
        .json(&Query {
            t: "validatorSummaries".to_string(),
        })
        .send()
        .await
        .context(format!("Error with the response from: {}", info_url))
        .map_err(|e| HypermonError::ResponseError(e))?
        .json::<Vec<Validator>>()
        .await
        .context("Error while deserializing Validator summaries")
        .map_err(|e| {
            error!("{e:?}");
            HypermonError::DeserializationError(e)
        })?;

    validators.sort_by(|a, b| b.stake.cmp(&a.stake));

    Ok(validators)
}
