use reqwest::Client;
use reqwest::StatusCode;
use std::str::FromStr;

use url::Url;

use crate::arseeding_types::{APIErrorRes, ASError};
use crate::everpay_types::TokenInfo;
use crate::everpay_types::DEFAULT_EVERPAY_URL;
use crate::everpay_types::{Balances, StatusRes, Transaction};

pub struct EverpayClient {
    client: Client,
    url: Url,
}

impl Default for EverpayClient {
    fn default() -> Self {
        EverpayClient {
            client: reqwest::Client::new(),
            url: Url::from_str(DEFAULT_EVERPAY_URL).unwrap(),
        }
    }
}

impl EverpayClient {
    pub fn new(client: reqwest::Client, url: Url) -> EverpayClient {
        Self { client, url }
    }

    pub fn set_client(&mut self, c: Client) {
        self.client = c
    }

    pub async fn info(&self) -> Result<TokenInfo, ASError> {
        let res = self.client.get(format!("{}info", self.url)).send().await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<TokenInfo>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn balances(&self, account_id: &str) -> Result<Balances, ASError> {
        let res = self
            .client
            .get(format!("{}balances/{}", self.url, account_id))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<Balances>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn submit_tx(&self, tx: &Transaction) -> Result<StatusRes, ASError> {
        let res = self
            .client
            .post(format!("{}{}", self.url, "tx"))
            .header("Content-Type", "application/json")
            .json(tx)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<StatusRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_balance() {
        let c = EverpayClient::default();

        let res = c
            .balances("2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0")
            .await;

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_info() {
        let c = EverpayClient::default();

        let res = c.info().await;

        println!("{:#?}", res);
    }
}
