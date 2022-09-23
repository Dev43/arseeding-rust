use std::{collections::HashMap, str::FromStr};

use crate::types::{
    APIErrorRes, ASError, BundlerRes, FeeRes, ItemMetaRes, ItemSubmissionRes, OrderRes,
    SubmitNativeRes, Tag,
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

    /*
        /bundle/orders/:signer
        [
        {
            "id": 13,
            "createdAt": "2022-07-11T04:07:12.261Z",
            "updatedAt": "2022-07-11T05:07:44.369Z",
            "itemId": "n6Xv8LwdpsQgpaTQgaXQfUORW-KxYePDnj-1ka9dHxM",
            "signer": "0x4002ED1a1410aF1b4930cF6c479ae373dEbD6223",
            "signType": 3,
            "size": 7802,
            "currency": "USDT",
            "decimals": 6,
            "fee": "817",
            "paymentExpiredTime": 1657516032,
            "expectedBlock": 972166,
            "paymentStatus": "expired",
            "paymentId": "",
            "onChainStatus": "failed"
        },
        ...
    ]Query Params:
    cursorId: (option) Return the id of the last record in the list, used for paging.
    Params:

    signer: The address corresponding to the signature private key, ecc or rsa address of bundle item.
        */
    pub async fn get_bundler_orders(
        &self,
        signer: &str,
        cursor: &str,
    ) -> Result<OrderRes, ASError> {
        let mut req = self
            .client
            .get(format!("{}bundle/orders/{}", self.url, signer));

        if cursor.len() > 0 {
            req = req.query(&["cursor", cursor]);
        }

        let res = req.send().await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<OrderRes>().await?),
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
    async fn it_runs() {
        // run()
        let c = ASClient::default();
        // let res = c.get_bundler().await.unwrap();
        // let res = c.get_bundle_fee("1000", "usdc").await.unwrap();
        // let res = c
        //     .get_bundler_orders("0x4002ED1a1410aF1b4930cF6c479ae373dEbD6223")
        //     .await
        //     .unwrap();
        // let res = c
        //     .get_item_meta("IlYC5sG61mhTOlG2Ued5LWxN4nuhyZh3ror0MBbPKy4")
        //     .await;
        let res = c
            .get_items_by_ar_id("-19XXEkalF_klxLLpknoTGAr6AnQMCgqzz-GjNn-oSE")
            .await;

        println!("{:?}", res);
    }
}
