use anyhow::Result;
use tonic::async_trait;

use crate::{geyser::SubscribeUpdateSlot, indexer::db_processor::db_processor, rpc::client::SolanaRpc, solana::storage::confirmed_block::{ConfirmedBlock, ConfirmedTransaction}, types::SlotMeta};

#[async_trait]
pub trait SlotProcessor{
    async fn process_slot(&self, slot: SubscribeUpdateSlot, rpc: &SolanaRpc, processor: &db_processor) -> Result<()>;
}