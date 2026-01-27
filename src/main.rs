use anyhow::Error;

use crate::{indexer::{processor::Backfiller, simple_processor::Simpleprocessor}, rpc::client::SolanaRpc};

pub mod types;
pub mod indexer;
pub mod rpc;
pub mod storage;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let rpc = SolanaRpc::new("https://solana-devnet.g.alchemy.com/v2/6EM78ky3wCNeN-LPwKPhaC_fuRz6v68N");
    let processor = Simpleprocessor::new();
    let backfiller = Backfiller::new(&rpc, &processor);

    let finalized_slot = rpc.get_finalized_slot().await?;

    println!("RPC finalized slot {}", finalized_slot);

    let start = finalized_slot.saturating_sub(5);
    println!("{}", start);
    backfiller.backfiller_range(start, finalized_slot).await?;

    Ok(())

}
