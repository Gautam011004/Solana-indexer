

use anyhow::{Error, Ok};
use reqwest::{Client};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::rpc::types::{Rpcblock, Rpcrequest, Rpcresponse};

pub struct SolanaRpc {
    client: Client,
    url: String,
    requestid: std::sync::atomic::AtomicU64 
}

impl SolanaRpc{
    pub fn new(url : impl Into<String>) -> Self{
        Self {
            client: Client::new(),
            url: url.into(),
            requestid: std::sync::atomic::AtomicU64::new(1)
        }
    }
    async fn send<Tparams, Tresults>(
        &self, method: &'static str, 
        params: Tparams) -> Result<Tresults, Error> 
        where 
            Tparams: serde::Serialize,
            Tresults: DeserializeOwned
        {
            let id = self
                                .requestid
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            
            let request = Rpcrequest {
                                                jsonrpc: "2.0",
                                                id,
                                                method,
                                                params
                                            };
            let response = self
                                        .client
                                        .post(&self.url)
                                        .json(&request)
                                        .send()
                                        .await?
                                        .error_for_status()?
                                        .json::<Rpcresponse<Tresults>>()
                                        .await?;
            Ok(response.result)
        }
        async fn get_finalized_slot(&self) -> Result<u64, Error> {
            let params = json!([{"commitment": "finalized"}]);

            let slot = self.send("getSlot", params).await?;
            Ok(slot)
        }
        pub async fn get_finalized_block(&self, slot: u64) -> Result<Rpcblock, Error> {
            let params = json!([
                    slot, {
                    "commitment" : "finalized",
                    "transactionDetails": "full",
                    "rewards": false
            }]);
            let block = self.send("getBlock", params).await?;
            Ok(block)
        }
}