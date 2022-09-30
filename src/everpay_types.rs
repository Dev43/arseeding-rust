use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

use crate::arseeding_types::ASError;
use async_trait::async_trait;

pub const TX_VERSION_V1: &str = "v1";

pub const TX_ACTION_TRANSFER: &str = "transfer";
pub const TX_ACTION_MINT: &str     = "mint";
pub const TX_ACTION_BURN: &str     = "burn";



pub const DEFAULT_EVERPAY_URL: &str = "https://api.everpay.io";


pub enum SignerType {
    ECDSA,
    RSA
}

#[async_trait]
pub trait Signer {
    async fn sign(&self, msg:&str) -> Result<String, ASError>;
    fn owner(&self) -> Result<String, ASError>;
    fn wallet_address(&self) -> Result<String, ASError>;
    fn signer_type(&self) -> SignerType;
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub accid: String,
    pub balances: Vec<Balance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub tag: String,
    pub amount: String,
    pub decimals: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub token_symbol: String,
    pub action: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub fee: String,
    pub fee_recipient: String,
    pub nonce: String,
    pub token_id: String,
    pub chain_type: String,
    pub chain_id: String,
    pub data: String,
    pub version: String,
    pub sig: String,
}

impl Transaction {
    pub fn sig_msg(&self) -> String {
       format!("tokenSymbol:{}\naction:{}\nfrom:{}\nto:{}\namount:{}\nfee:{}\nfeeRecipient:{}\nnonce:{}\ntokenID:{}\nchainType:{}\nchainID:{}\ndata:{}\nversion:{}", 
        self.token_symbol,
        self.action,
        self.from,
        self.to,
        self.amount,
        self.fee, 
        self.fee_recipient,
        self.nonce,
        self.token_id,
        self.chain_type,
        self.chain_id,
        self.data,
        self.version
    )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRes {
    pub status: String,
    pub tx: Transaction,
    pub ever_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusRes {
    pub status: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayTxData {
    pub app_name: String,
    pub action: String, 
    pub item_ids: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub is_synced: bool,
    pub is_closed: bool,
    pub balance_root_hash: String,
    pub root_hash: String,
    pub ever_root_hash: String,
    pub owner: String,
    #[serde(rename = "ethChainID")]
    pub eth_chain_id: String,
    pub fee_recipient: String,
    pub eth_locker: String,
    pub ar_locker: String,
    pub lockers: HashMap<String, String>,
    pub token_list: Vec<TokenList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenList {
    pub tag: String,
    pub id: String,
    pub symbol: String,
    pub decimals: i64,
    pub total_supply: String,
    pub chain_type: String,
    #[serde(rename = "chainID")]
    pub chain_id: String,
    pub burn_fees: HashMap<String, String>,
    pub transfer_fee: String,
    pub bundle_fee: String,
    pub holder_num: i64,
    pub cross_chain_info_list: HashMap<String, CrossChainInfoListDetails>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossChainInfoListDetails {
    pub target_chain_id: String,
    pub target_chain_type: String,
    pub target_decimals: i64,
    pub target_token_id: String,
}

