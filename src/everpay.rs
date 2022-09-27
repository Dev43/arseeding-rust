use std::str::FromStr;

use arloader::transaction::Base64;
use arloader::Arweave;
use chrono::Utc;
use reqwest::Client;
use reqwest::StatusCode;

use url::Url;

use crate::everpay_types::StatusRes;
use crate::everpay_types::Transaction;
use crate::everpay_types::TX_VERSION_V1;
use crate::{
    everpay_types::Balances,
    types::{APIErrorRes, ASError},
};
pub struct EverpayClient {
    client: Client,
    arweave: Arweave,
    url: Url,
}

const DEFAULT_URL: &str = "https://api.everpay.io";

impl Default for EverpayClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            arweave: Arweave::default(),
            url: Url::from_str(DEFAULT_URL).unwrap(),
        }
    }
}

impl EverpayClient {
    pub fn new(arweave: Arweave, client: reqwest::Client, url: Url) -> EverpayClient {
        Self {
            arweave,
            client,
            url,
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

    pub async fn sign_and_send_tx(
        &self,
        token_symbol: &str,
        action: &str,
        fee: u64,
        fee_recipient: &str,
        token_id: &str,
        chain_type: &str,
        chain_id: &str,
        receiver: &str,
        amount: u64,
        data: &str,
    ) -> Result<StatusRes, ASError> {
        let mut tx = Transaction {
            token_symbol: token_symbol.to_string(),
            action: action.to_string(),
            from: self.arweave.crypto.wallet_address().unwrap().to_string(),
            to: receiver.to_string(),
            amount: amount.to_string(),
            fee: fee.to_string(),
            fee_recipient: fee_recipient.to_string(),
            nonce: self.get_nonce().to_string(),
            token_id: token_id.to_string(),
            chain_type: chain_type.to_string(),
            chain_id: chain_id.to_string(),
            data: data.to_string(),
            version: TX_VERSION_V1.to_string(),
            sig: "".to_string(),
        };

        println!("{}", tx.sig_msg());
        // first hash the message (using Eth prefix message)

        let eth_hash = ethers::utils::hash_message(tx.sig_msg());

        // now hash normally (the lib used in arloader does not pre hash the message)
        let msg = self
            .arweave
            .crypto
            .hash_sha256(eth_hash.as_bytes())
            .unwrap();

        // now we sign (the lib used)
        // let sig = self.arweave.crypto.sign(eth_hash.as_bytes()).unwrap();
        let sig = self.arweave.crypto.sign(&msg).unwrap();

        tx.sig = Base64(sig).to_string();

        println!("{:?}", tx);

        self.submit_tx(&tx).await
        // Ok(StatusRes {
        //     status: "ok".to_string(),
        // })
    }

    fn get_nonce(&self) -> i64 {
        Utc::now().timestamp_nanos() / 1000000
    }
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use crate::everpay_types::{
        ACCOUNT_TYPE_AR, ARWEAVE_CHAIN_ID, CHAIN_ID, CHAIN_TYPE, TX_ACTION_TRANSFER,
    };

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
    async fn it_signs_and_sends_tx() {
        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/test-----arweave-keyfile-2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let c = EverpayClient::new(
            arweave,
            reqwest::Client::new(),
            Url::from_str(DEFAULT_URL).unwrap(),
        );

        let res = c
            .sign_and_send_tx(
                "AR",
                TX_ACTION_TRANSFER,
                0,
                "0x6451eB7f668de69Fb4C943Db72bCF2A73DeeC6B1",
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA,0x4fadc7a98f2dc96510e42dd1a74141eeae0c1543",
                CHAIN_TYPE,
                CHAIN_ID,
                "rQ3VdxFnCOYjquTF88UANCax8-viPtrmu5TA2dktQlY",
                1,
                r#"{"hello":"world","this":"is everpay"}"#,
            )
            .await;

        println!("{:#?}", res);
    }
}
