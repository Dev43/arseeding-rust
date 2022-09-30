use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;
use std::fmt::Display;

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
    pub id: u64,
    #[serde(default, deserialize_with = "option_datefmt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "option_datefmt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub item_id: String,
    pub signer: String,
    pub sign_type: u8,
    pub size: i64,
    pub currency: String,
    pub decimals: u8,
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
    ArLoaderError(arloader::error::Error),
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
            ASError::ArLoaderError( e ) => write!(f, "arloader: {}", e)
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

impl From<arloader::error::Error> for ASError {
    fn from(e: arloader::error::Error) -> Self {
        ASError::ArLoaderError(e)
    }
}

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
// 2022-06-24T03:29:54.174Z

fn datefmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

fn option_datefmt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "datefmt")] DateTime<Utc>);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}
