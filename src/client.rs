use std::{collections::HashMap, str::FromStr};

use crate::types::{
    APIErrorRes, ASError, BundlerRes, FeeRes, ItemMetaRes, ItemSubmissionRes, OrderRes,
    SubmitNativeRes,
};
use arloader::Arweave;
use reqwest::{Client, StatusCode};

use url::Url;
pub struct ASClient {
    client: Client,
    arweave: Arweave,
    url: Url,
}

const DEFAULT_URL: &str = "https://arseed.web3infra.dev";

impl Default for ASClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            arweave: Arweave::default(),
            url: Url::from_str(DEFAULT_URL).unwrap(),
        }
    }
}

impl ASClient {
    pub fn new(url: Url, client: Client, arweave: Arweave) -> Self {
        ASClient {
            url,
            client,
            arweave,
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

    pub async fn submit_item(
        &self,
        data: Vec<u8>,
        currency: &str,
        api_key: &str,
    ) -> Result<ItemSubmissionRes, ASError> {
        // TODO check currency

        let mut url: String = "/bundle/tx".to_string();
        if currency.len() > 0 {
            url = format!("{}{}{}", self.url, "/bundle/tx/", currency);
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

    /*
        /bundle/data
        Header: X-API-KEY: 'your apiKey'
    Body: --data-binary 'data'

    {
        itemId: "tSB2-PS3Qr-POmBgjIoi4wRYhhGq3UZ9uPO8dUf2LhM"
    }

        */
    pub async fn submit_native_data(
        &self,
        data: Vec<u8>,
        content_type: &str,
        tags: &HashMap<String, String>,
        api_key: &str,
    ) -> Result<SubmitNativeRes, ASError> {
        let mut req = self
            .client
            .post(format!("{}{}", self.url, "/bundle/data"))
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
    use super::*;

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_runs() {
        // run()
        let c = ASClient::default();

        let res = c
            .get_items_by_ar_id("-19XXEkalF_klxLLpknoTGAr6AnQMCgqzz-GjNn-oSE")
            .await;

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_bundlr() {
        // run()
        let c = ASClient::default();
        let res = c.get_bundler().await.unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_fee() {
        // run()
        let c = ASClient::default();
        let res = c.get_bundle_fee("1000", "USDC").await.unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_fetches_orders() {
        // run()
        let c = ASClient::default();
        let res = c
            .get_bundler_orders("Ii5wAMlLNz13n26nYY45mcZErwZLjICmYd46GZvn4ck", "")
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "outbound_calls"]
    async fn it_gets_item_meta() {
        // run()
        let c = ASClient::default();
        let res = c
            .get_item_meta("_mbWucSl6nB6yagI7NaR8CO8UR7C9tvizO1V4i6Vck0")
            .await
            .unwrap();

        println!("{:#?}", res);
    }
}
