use std::fmt::Display;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIErrorRes {
    pub error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundlerRes {
    pub bundler: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSubmissionRes {
    pub item_id: String,
    pub bundler: String,
    pub currency: String,
    pub decimals: i64,
    pub fee: String,
    pub payment_expired_time: i64,
    pub expected_block: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitNativeRes {
    pub item_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRes {
    pub currency: String,
    pub decimals: i64,
    pub final_fee: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRes {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub item_id: String,
    pub signer: String,
    pub sign_type: i64,
    pub size: i64,
    pub currency: String,
    pub decimals: i64,
    pub fee: String,
    pub payment_expired_time: i64,
    pub expected_block: i64,
    pub payment_status: String,
    pub payment_id: String,
    pub on_chain_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemMetaRes {
    pub signature_type: i64,
    pub signature: String,
    pub owner: String,
    pub target: String,
    pub anchor: String,
    pub tags: Vec<Tag>,
    pub data: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum ASError {
    ArgumentError { arg: String },
    URLError { url: String },
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
    APIError { e: String },
    // RingError(Unspecified),
}

impl Display for ASError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASError::ArgumentError { arg } => write!(f, "argument not valid: {}", arg),
            ASError::URLError { url } => write!(f, "invalid url: {}", url),
            ASError::APIError { e } => write!(f, "api: {}", e),
            ASError::ReqwestError(e) => write!(f, "reqwest: {}", e),
            ASError::IOError(e) => write!(f, "io: {}", e),
            // ASError::ParseIntError(e) => write!(f, "parse int error: {}", e),
            // ASError::RingError(e) => write!(f, "ring error: {}", e),
        }
    }
}

impl From<reqwest::Error> for ASError {
    fn from(e: reqwest::Error) -> Self {
        ASError::ReqwestError(e)
    }
}

impl ASError {
    pub fn api_error(e: &str) -> ASError {
        ASError::APIError { e: e.to_string() }
    }
}
