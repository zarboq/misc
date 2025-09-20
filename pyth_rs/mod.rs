use reqwest::{Client, RequestBuilder};

use super::ApiClient;
use crate::error::Result;

pub mod models;
use models::PriceUpdate;

pub struct Pyth {
    base_url: String,
    client: Client,
}

#[async_trait::async_trait]
impl ApiClient for Pyth {
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn customize(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }
}

#[derive(Debug)]
pub struct PriceParams {
    pub ids: Vec<u64>,
    pub timestamp: u64,
}

impl PriceParams {
    pub fn new(ids: Vec<u64>, timestamp: u64) -> Self {
        Self { ids, timestamp }
    }
}

#[derive(Debug)]
pub struct LatestParams {
    pub ids: Vec<u64>,
}

impl Pyth {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    pub async fn get_price(&self, params: PriceParams) -> Result<PriceUpdate> {
        let route = "updates/price/";
        let mut req = self.get(route);
        let mut query_pairs = params
            .ids
            .into_iter()
            // hex formatting still works for u64
            .map(|id| ("ids[]".to_string(), format!("{id:x}")))
            .collect::<Vec<_>>();
        query_pairs.push(("timestamp".to_string(), params.timestamp.to_string()));
        req = req.query(&query_pairs);
        self.execute(req).await
    }

    pub async fn get_latest_price(&self, ids: Vec<u64>) -> Result<PriceUpdate> {
        let route = "updates/price/latest";
        let mut req = self.get(route);
        let query_pairs = ids
            .into_iter()
            .map(|id| ("ids[]".to_string(), format!("{id:x}")))
            .collect::<Vec<_>>();
        req = req.query(&query_pairs);
        self.execute(req).await
    }
}
