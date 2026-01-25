use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rpcrequest<T> {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: &'static str,
    pub params: T
}

#[derive(Serialize,Deserialize)]
pub struct Rpcresponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u64
}

#[derive(Serialize,Deserialize)]
pub struct RpcError{
    pub code: u64,
    pub message: String
}

#[derive(Serialize,Deserialize)]
pub struct Slotresponse(pub u64);

#[derive(Serialize,Deserialize)]
pub struct Rpcblock{
    pub blockheight: Option<u64>,
    pub parentSlot: u64,
    pub transactions: Option<Vec<RpcTransaction>>
}

#[derive(Serialize,Deserialize)]
pub struct RpcTransaction{
    pub meta: Option<RpcTransactionmeta>,
    pub transaction: RpcTransactionData
}

#[derive(Serialize,Deserialize)]
pub struct RpcTransactionmeta{
    pub err: Option<serde_json::Value>,
}

#[derive(Serialize,Deserialize)]
pub struct RpcTransactionData{
    pub signatures: Vec<String>
}