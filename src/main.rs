use anyhow::Error;
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
    Ok(())
}
