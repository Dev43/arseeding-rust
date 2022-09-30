use std::collections::HashMap;

use arloader::transaction::Base64;
use arloader::Arweave;
use async_trait::async_trait;
use chrono::Utc;
use walletconnect::{self, qr};

use crate::arseeding_types::ASError;
use crate::everpay_client::EverpayClient;
use crate::everpay_types::Signer;
use crate::everpay_types::TokenInfo;
use crate::everpay_types::TokenList;
use crate::everpay_types::{Balances, SignerType, StatusRes, Transaction, TX_VERSION_V1};

pub struct Everpay<'a> {
    client: EverpayClient,
    signer: &'a dyn Signer,
    tokens: HashMap<String, TokenList>,
    symbol_to_tag: HashMap<String, String>,
}

impl<'a> Everpay<'a> {
    pub async fn new(client: EverpayClient, signer: &'a dyn Signer) -> Result<Everpay, ASError> {
        let mut c = Self {
            client,
            signer,
            tokens: HashMap::new(),
            symbol_to_tag: HashMap::new(),
        };

        c.update_info().await?;

        Ok(c)
    }

    async fn update_info(&mut self) -> Result<(), ASError> {
        let token_info = self.client.info().await?;

        let mut tokens = HashMap::new();
        let mut sym_to_tags = HashMap::new();

        for t in token_info.token_list {
            let tag = t.tag.clone();
            let tag_2 = t.tag.clone();
            let symbol = t.symbol.clone().to_lowercase();
            tokens.insert(tag, t);
            sym_to_tags.insert(symbol, tag_2);
        }
        self.tokens = tokens;
        self.symbol_to_tag = sym_to_tags;

        Ok(())
    }

    pub async fn info(&self) -> Result<TokenInfo, ASError> {
        self.client.info().await
    }

    pub fn symbol_to_tag(&self) -> HashMap<String, String> {
        self.symbol_to_tag.clone()
    }

    pub fn tokens(&self) -> HashMap<String, TokenList> {
        self.tokens.clone()
    }

    pub async fn balances(&self, account_id: &str) -> Result<Balances, ASError> {
        self.client.balances(account_id).await
    }

    pub async fn submit_tx(&self, tx: &Transaction) -> Result<StatusRes, ASError> {
        self.client.submit_tx(tx).await
    }

    pub async fn sign(&self, msg: &str) -> Result<String, ASError> {
        self.signer.sign(msg).await
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

        tx.sig = self.sign(&tx.sig_msg()).await?;

        self.submit_tx(&tx).await
    }

    fn get_nonce(&self) -> i64 {
        Utc::now().timestamp_nanos() / 1000000
    }
}

pub struct ArweaveSigner {
    arweave: Arweave,
}

impl ArweaveSigner {
    pub fn new(arweave: Arweave) -> impl Signer {
        Self { arweave }
    }
}

#[async_trait]
impl Signer for ArweaveSigner {
    async fn sign(&self, msg: &str) -> Result<String, ASError> {
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

pub struct EthSigner {
    client: walletconnect::Client,
    account: String,
}

impl EthSigner {
    pub async fn new(client: walletconnect::Client) -> impl Signer {
        let (accounts, _) = client.ensure_session(qr::print).await.unwrap();
        Self {
            client,
            account: format!("{:?}", accounts[0]),
        }
    }
}

#[async_trait]
impl Signer for EthSigner {
    async fn sign(&self, msg: &str) -> Result<String, ASError> {
        // first hash the message (using Eth prefix message)

        let sig = self
            .client
            .personal_sign(&[msg, &self.account])
            .await
            .unwrap();

        Ok(format!("{}", sig,))
    }
    fn owner(&self) -> Result<String, ASError> {
        Ok("".to_string())
    }
    fn signer_type(&self) -> SignerType {
        SignerType::ECDSA
    }
    fn wallet_address(&self) -> Result<String, ASError> {
        Ok(self.account.clone())
    }
}

#[cfg(test)]
mod test {

    use std::{path::PathBuf, str::FromStr};

    use crate::everpay_types::{CHAIN_ID, CHAIN_TYPE, TX_ACTION_TRANSFER};
    use url::Url;
    use walletconnect::Metadata;

    use super::*;

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_info() {
        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/test-----arweave-keyfile-2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let signer = ArweaveSigner::new(arweave);
        let c = Everpay::new(EverpayClient::default(), &signer)
            .await
            .unwrap();

        println!("{:#?}", c.symbol_to_tag());
        println!("{:#?}", c.tokens());
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_signs_and_sends_tx_eth() {
        let c = walletconnect::Client::new(
            "arseeding",
            Metadata {
                description: "Arseeding".into(),
                url: "https://github.com/nlordell/walletconnect-rs"
                    .parse()
                    .unwrap(),
                icons: vec!["https://avatars0.githubusercontent.com/u/4210206"
                    .parse()
                    .unwrap()],
                name: "Arseeding".into(),
            },
        )
        .unwrap();

        let signer = EthSigner::new(c).await;

        let c = Everpay::new(EverpayClient::default(), &signer)
            .await
            .unwrap();

        let res = c
            .sign_and_send_tx(
                "AR",
                TX_ACTION_TRANSFER,
                0,
                "0x6451eB7f668de69Fb4C943Db72bCF2A73DeeC6B1",
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA,0x4fadc7a98f2dc96510e42dd1a74141eeae0c1543",
                CHAIN_TYPE,
                CHAIN_ID,
                "2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0",
                1,
                r#"{"hello":"world","this":"is everpay"}"#,
            )
            .await;

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_signs_and_sends_tx_arweave() {
        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/test-----arweave-keyfile-2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let signer = ArweaveSigner::new(arweave);

        let c = Everpay::new(EverpayClient::default(), &signer)
            .await
            .unwrap();

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
