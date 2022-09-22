use std::str::FromStr;

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

    // https://arseed.web3infra.dev/bundle/bundler
    /*
        {
        "bundler": "uDA8ZblC-lyEFfsYXKewpwaX-kkNDDw8az3IW9bDL68"
    }
        */
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

    // /bundle/tx/:currency (either AR or USDC
    // Headers: Content-Type:application/octet-stream
    // Body: --data-binary 'bundle item data')

    /*
        {
        itemId:             "tSB2-PS3Qr-POmBgjIoi4wRYhhGq3UZ9uPO8dUf2LhM",
        bundler:            "Fkj5J8CDLC9Jif4CzgtbiXJBnwXLSrp5AaIllleH_yY",
        currency:           "AR",
        decimals:           12,
        fee:                "113123",
        paymentExpiredTime: 122132421,
        expectedBlock:      3144212,

    }
    if api key add     Header: X-API-KEY: 'your apiKey'
        */
    pub async fn submit_item(&self) {}

    /*
        /bundle/data
        Header: X-API-KEY: 'your apiKey'
    Body: --data-binary 'data'

    {
        itemId: "tSB2-PS3Qr-POmBgjIoi4wRYhhGq3UZ9uPO8dUf2LhM"
    }

        */
    pub async fn submit_native_data(&self) {}

    /*
        bundle/fee/:size/:currency
        {
        "currency": "USDC",
        "decimals": 6,
        "finalFee": "4413"
    }
        */

    // TODO validate currencies + size
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
    pub async fn get_bundler_orders(&self, signer: &str) -> Result<OrderRes, ASError> {
        let res = self
            .client
            .get(format!("{}bundle/orders/{}", self.url, signer))
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => return Ok(res.json::<OrderRes>().await?),
            _ => {
                return Err(ASError::APIError {
                    e: res.json::<APIErrorRes>().await?.error,
                })
            }
        }
    }

    /*
    URL: /bundle/tx/:itemId
    Params:

    itemId: bundle item Id

    {
        "signatureType": 3,
        "signature": "DC469T6Fz3ByFCtEjnP9AdsLSDFqINxvbIqFw1qwk0ApHtpmytRWFHZeY2gBN9nXopzY7Sbi9u5U6UcpPrwPlxs",
        "owner": "BCLR8qIeP8-kDAO6AifvSSzyCQJBwAtPYErCaX1LegK7GwXmyMvhzCmt1x6vLw4xixiOrI34ObhU2e1RGW5YNXo",
        "target": "",
        "anchor": "",
        "tags": [
            {
              "name": "a",
              "value": "aa"
            },
            {
              "name": "b",
              "value": "bb"
            },
            {
              "name": "Content-Type",
              "value": "image/png"
            }
          ],
        "data": "",
        "id": "IlYC5sG61mhTOlG2Ued5LWxN4nuhyZh3ror0MBbPKy4"
    }

    */
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

    /*

        hod: GET
    URL: /bundle/itemIds/:arId
    [
        "ikLaIRdrzTDD5-nL7C7LeWgZr_XEXbB5dB3C_hJZxXE",
        "x0gzOsOtos4X5vzoJ1CW9wq2pMPB7q7v_zjnvPPNjp0"
    ]
        */
    pub async fn get_items_by_ar_id(&self) {}
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
        let res = c
            .get_item_meta("IlYC5sG61mhTOlG2Ued5LWxN4nuhyZh3ror0MBbPKy4")
            .await;

        println!("{:?}", res);
    }
}
