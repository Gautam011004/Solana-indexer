use anyhow::Error;

use crate::{indexer::{processor::Backfiller, simple_processor::Simpleprocessor}, rpc::client::SolanaRpc};

pub mod types;
pub mod indexer;
pub mod rpc;
pub mod storage;
pub mod stream;

pub mod geyser {
    tonic::include_proto!("geyser");
}
pub mod solana {
    pub mod storage{
        pub mod confirmed_block {
        tonic::include_proto!("solana.storage.confirmed_block");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = stream::client::GrpcClient::connect("https://solana-rpc.parafi.tech:10443").await?;

    client.subscribe().await?;

    Ok(())

}
