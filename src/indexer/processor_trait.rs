use anyhow::Result;
use tonic::async_trait;

use crate::{geyser::{SubscribeUpdateBlock, SubscribeUpdateSlot}, indexer::db_processor::db_processor, rpc::client::SolanaRpc};

#[async_trait]
pub trait SlotProcessor{
    async fn process_slot(&self, slot: SubscribeUpdateSlot, rpc: &SolanaRpc, processor: &db_processor) -> Result<()>;
    async fn process_block(&self, block: &SubscribeUpdateBlock) -> Result<()>;
}