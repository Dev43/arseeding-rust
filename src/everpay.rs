use std::str::FromStr;

use arloader::transaction::Base64;
use arloader::Arweave;
use chrono::Utc;
use reqwest::Client;
use reqwest::StatusCode;

use url::Url;

use crate::everpay_types::Signer;
use crate::everpay_types::{Balances, SignerType, StatusRes, Transaction, TX_VERSION_V1};
use crate::types::{APIErrorRes, ASError};
pub struct EverpayClient<'a> {
    client: Client,
    url: Url,
    signer: &'a dyn Signer,
}

pub struct ArweaveSigner {
    arweave: Arweave,
}

impl ArweaveSigner {
    fn new(arweave: Arweave) -> impl Signer {
        Self { arweave }
    }
}

impl Signer for ArweaveSigner {
    fn sign(&self, msg: &[u8]) -> Result<String, ASError> {
        // first hash the message (using Eth prefix message)
        let eth_hash = ethers::utils::hash_message(msg);

        let sig = self.arweave.crypto.sign(eth_hash.as_bytes())?;

        Ok(format!(
            "{},{}",
            Base64(sig).to_string(),
            self.arweave.crypto.keypair_modulus()?.to_string()
        ))
    }
    fn owner(&self) -> Result<String, ASError> {
        let r = self.arweave.crypto.keypair_modulus()?;
        Ok(r.to_string())
    }
    fn signer_type(&self) -> SignerType {
        SignerType::RSA
    }
    fn wallet_address(&self) -> Result<String, ASError> {
        let addr = self.arweave.crypto.wallet_address()?;

        Ok(addr.to_string())
    }
}

const DEFAULT_URL: &str = "https://api.everpay.io";

impl<'a> EverpayClient<'a> {
    pub fn new(client: reqwest::Client, url: Url, signer: &'a dyn Signer) -> EverpayClient {
        Self {
            client,
            url,
            signer,
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

    pub fn sign(&self, msg: &str) -> Result<String, ASError> {
        self.signer.sign(msg.as_bytes())
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
            from: self.signer.wallet_address()?,
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

        tx.sig = self.sign(&tx.sig_msg())?;

        println!("{}", tx.sig);

        self.submit_tx(&tx).await
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
        let signer = ArweaveSigner::new(Arweave::default());
        let c = EverpayClient::new(
            reqwest::Client::new(),
            Url::from_str(DEFAULT_URL).unwrap(),
            &signer,
        );

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

        let signer = ArweaveSigner::new(arweave);

        let c = EverpayClient::new(
            reqwest::Client::new(),
            Url::from_str(DEFAULT_URL).unwrap(),
            &signer,
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
