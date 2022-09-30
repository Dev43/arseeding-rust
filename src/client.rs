use std::collections::HashMap;

use crate::{
    arseeding_types::{
        APIErrorRes, ASError, BundlerRes, FeeRes, ItemMetaRes, ItemSubmissionRes, OrderRes,
        SubmitNativeRes,
    },
    everpay::Everpay,
    everpay_types::PayTxData,
};
use arloader::{
    transaction::{FromUtf8Strs, Tag},
    Arweave,
};
use reqwest::{Client, StatusCode};

use url::Url;
pub struct ASClient {
    client: Client,
    arweave: Arweave,
    url: Url,
    everpay: Everpay,
}

const DEFAULT_ARSEEDING_URL: &str = "https://arseed.web3infra.dev";

impl ASClient {
    pub fn new(url: Url, client: Client, arweave: Arweave, everpay: Everpay) -> Self {
        ASClient {
            url,
            client,
            arweave,
            everpay,
        }
    }

    pub fn set_client(mut self, c: Client) {
        self.client = c;
    }

    pub async fn get_bundler(&self) -> Result<BundlerRes, ASError> {
        let res = self
            .client
            .get(format!("{}{}", self.url, "bundle/bundler"))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<BundlerRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn bundle_and_submit(
        &self,
        data: Vec<u8>,
        tags: &HashMap<String, String>,
        currency: &str,
        api_key: &str,
    ) -> Result<ItemSubmissionRes, ASError> {
        let t: Vec<Tag<String>> = tags
            .iter()
            .map(|(k, v)| Tag::from_utf8_strs(k, v).unwrap())
            .collect();

        let data_item = self.arweave.create_data_item(data, t, true)?;
        let signed = self.arweave.sign_data_item(data_item)?;

        self.submit_item(signed.serialize()?, currency, api_key)
            .await
    }

    pub async fn send_and_pay(
        &self,
        currency: &str,
        tags: &HashMap<String, String>,
        data: Vec<u8>,
        api_key: &str,
    ) -> Result<String, ASError> {
        let order = self
            .bundle_and_submit(data, tags, currency, api_key)
            .await?;

        let order_id = order.item_id;

        // pay for tx using everpay
        let fee = order.fee;
        let fee_int: u64 = fee.parse().unwrap();
        let bundler = order.bundler;
        let currency = order.currency;

        let data = serde_json::to_string(&PayTxData {
            app_name: String::from("arseeding"),
            action: String::from("payment"),
            item_ids: vec![order_id.clone()],
        })
        .unwrap();

        self.everpay
            .transfer(&currency, &bundler, fee_int, &data)
            .await?;

        Ok(order_id)
    }

    pub async fn submit_item(
        &self,
        data: Vec<u8>,
        currency: &str,
        api_key: &str,
    ) -> Result<ItemSubmissionRes, ASError> {
        // TODO check currency

        let mut url: String = "bundle/tx".to_string();
        if currency.len() > 0 {
            url = format!("{}{}{}", self.url, "bundle/tx/", currency);
        }

        let mut req = self
            .client
            .post(url)
            .header("Content-Type", "application/octet-stream")
            .body(data);

        if api_key.len() > 0 {
            req = req.header("X-API-KEY", api_key);
        }

        let res = req.send().await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<ItemSubmissionRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn submit_native_data(
        &self,
        data: Vec<u8>,
        content_type: &str,
        tags: &HashMap<String, String>,
        api_key: &str,
    ) -> Result<SubmitNativeRes, ASError> {
        let mut req = self
            .client
            .post(format!("{}{}", self.url, "bundle/data"))
            .header("Content-Type", content_type)
            .query(&["Content-Type", content_type])
            .query(tags)
            .body(data);

        if api_key.len() > 0 {
            req = req.header("X-API-KEY", api_key);
        }

        let res = req.send().await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<SubmitNativeRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn get_bundle_fee(&self, size: &str, currency: &str) -> Result<FeeRes, ASError> {
        let res = self
            .client
            .get(format!("{}bundle/fee/{}/{}", self.url, size, currency))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<FeeRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn get_bundler_orders(
        &self,
        signer: &str,
        cursor: &str,
    ) -> Result<Vec<OrderRes>, ASError> {
        let mut req = self
            .client
            .get(format!("{}bundle/orders/{}", self.url, signer));

        if cursor.len() > 0 {
            req = req.query(&["cursor", cursor]);
        }

        let res = req.send().await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<Vec<OrderRes>>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn get_item_meta(&self, item_id: &str) -> Result<ItemMetaRes, ASError> {
        let res = self
            .client
            .get(format!("{}bundle/tx/{}", self.url, item_id))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<ItemMetaRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    pub async fn get_items_by_ar_id(&self, ar_id: &str) -> Result<Vec<String>, ASError> {
        let res = self
            .client
            .get(format!("{}bundle/itemIds/{}", self.url, ar_id))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<Vec<String>>().await?),
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

    use crate::{everpay::ArweaveSigner, everpay_client::EverpayClient, everpay_types::Signer};
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    use super::*;

    async fn init_default<'a>(signer: Arc<dyn Signer>, arweave: Arweave) -> ASClient {
        let everpay = Everpay::new(EverpayClient::default(), signer)
            .await
            .unwrap();
        ASClient::new(
            Url::from_str(DEFAULT_ARSEEDING_URL).unwrap(),
            reqwest::Client::new(),
            arweave,
            everpay,
        )
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_runs() {
        let signer = Arc::new(ArweaveSigner::new(Arweave::default()));
        let ar = Arweave::default();
        let c = init_default(signer, ar).await;
        let res = c
            .get_items_by_ar_id("-19XXEkalF_klxLLpknoTGAr6AnQMCgqzz-GjNn-oSE")
            .await;

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_bundlr() {
        let signer = Arc::new(ArweaveSigner::new(Arweave::default()));

        let ar = Arweave::default();

        let c = init_default(signer, ar).await;
        let res = c.get_bundler().await.unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_fee() {
        let ar = Arweave::default();
        let signer = Arc::new(ArweaveSigner::new(Arweave::default()));
        let c = init_default(signer, ar).await;
        let res = c.get_bundle_fee("1000", "USDC").await.unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_fetches_orders() {
        let ar = Arweave::default();
        let signer = Arc::new(ArweaveSigner::new(Arweave::default()));
        let c = init_default(signer, ar).await;
        let res = c
            .get_bundler_orders("2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0", "")
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_item_meta() {
        let signer = Arc::new(ArweaveSigner::new(Arweave::default()));
        let ar = Arweave::default();
        let c = init_default(signer, ar).await;
        let res = c
            .get_item_meta("BewjUEppPQ9pljVrjMxF7A2Kkz5ZJt_Q7tXRkQDm2VQ")
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_bundles_and_submits() {
        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/arweave-key-7eV1qae4qVNqsNChg3Scdi-DpOLJPCogct4ixoq1WNg.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let signer = Arc::new(ArweaveSigner::new(arweave));

        let everpay = Everpay::new(EverpayClient::default(), signer)
            .await
            .unwrap();

        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/arweave-key-7eV1qae4qVNqsNChg3Scdi-DpOLJPCogct4ixoq1WNg.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let c = ASClient::new(
            Url::from_str(DEFAULT_ARSEEDING_URL).unwrap(),
            Client::new(),
            arweave,
            everpay,
        );

        let mut tags = HashMap::new();
        tags.insert("hello".to_string(), "there".to_string());

        let res = c
            .bundle_and_submit("test".as_bytes().to_vec(), &tags, "usdc", "")
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_bundles_submits_and_pays() {
        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/test-----arweave-keyfile-2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let signer = Arc::new(ArweaveSigner::new(arweave));

        let everpay = Everpay::new(EverpayClient::default(), signer)
            .await
            .unwrap();

        let arweave = Arweave::from_keypair_path(
            PathBuf::from(
                "./tests/fixtures/test-----arweave-keyfile-2NbYHgsuI8uQcuErDsgoRUCyj9X2wZ6PBN6WTz9xyu0.json",
            ),
            Url::from_str("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        let c = ASClient::new(
            Url::from_str(DEFAULT_ARSEEDING_URL).unwrap(),
            Client::new(),
            arweave,
            everpay,
        );

        let mut tags = HashMap::new();
        tags.insert("hello".to_string(), "there".to_string());

        let res = c
            .send_and_pay("ar", &tags, "test1".as_bytes().to_vec(), "")
            .await
            .unwrap();

        println!("{:#?}", res);
    }
}
