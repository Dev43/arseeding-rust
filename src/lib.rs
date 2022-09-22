use arloader::Arweave;
use reqwest::Client;

pub struct ASClient {
    client: Client,
    arweave: Arweave,
}

impl Default for ASClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            arweave: Arweave::default(),
        }
    }
}

impl ASClient {
    pub fn new(client: Client, arweave: Arweave) -> Self {
        ASClient { client, arweave }
    }

    pub fn set_client(mut self, c: Client) {
        self.client = c;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_runs() {
        // run()
        let _ = ASClient::default();
    }
}
