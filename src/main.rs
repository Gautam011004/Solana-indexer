use anyhow::Error;
pub mod types;
pub mod indexer;
pub mod rpc;
pub mod storage;
pub mod stream;

use std::env;

use crate::indexer::db_processor::db_processor;
use crate::rpc::client::SolanaRpc;
use crate::storage::postgres::PostgresStorage;
use crate::stream::client::GrpcClient;

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
    let _ = dotenvy::dotenv();

    let database_url =
        env::var("DATABASE_URL").map_err(|_| Error::msg("Missing env var DATABASE_URL"))?;
    let geyser_endpoint =
        env::var("GEYSER_ENDPOINT").map_err(|_| Error::msg("Missing env var GEYSER_ENDPOINT"))?;
    let rpc_endpoint =
        env::var("RPC_ENDPOINT").map_err(|_| Error::msg("Missing env var RPC_ENDPOINT"))?;

    println!("Starting indexer");
    println!("GEYSER_ENDPOINT={}", geyser_endpoint);
    println!("RPC_ENDPOINT={}", rpc_endpoint);

    let storage = PostgresStorage::new(&database_url).await?;
    let processor = db_processor::new(storage).await?;

    let rpc = SolanaRpc::new(rpc_endpoint);
    let finalized_slot = rpc.get_finalized_slot().await?;
    println!("RPC connectivity OK. Latest finalized slot: {}", finalized_slot);

    let mut grpc = GrpcClient::connect(&geyser_endpoint).await?;
    grpc.subscribe(&processor, &rpc).await?;
    Ok(())
}
