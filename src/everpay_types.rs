use serde_derive::Deserialize;
use serde_derive::Serialize;


pub const TX_VERSION_V1: &str = "v1";

pub const TX_ACTION_TRANSFER: &str = "transfer";
pub const TX_ACTION_MINT: &str     = "mint";
pub const TX_ACTION_BURN: &str     = "burn";

pub const AR_ADDRESS: &str   = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
pub const EVM_ADDRESS: &str  = "0x0000000000000000000000000000000000000000";
pub const ETH_ADDRESS: &str  = "0x0000000000000000000000000000000000000000";

pub const ACCOUNT_TYPE_AR: &str= "arweave";
pub const ARWEAVE_CHAIN_ID: &str = "0";

pub const ACCOUNT_TYPE_EVM: &str= "ethereum";
pub const ETH_CHAIN_ID: &str = "1";

pub const CHAIN_TYPE: &str = "arweave,ethereum";
pub const CHAIN_ID: &str = "0,1";



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
