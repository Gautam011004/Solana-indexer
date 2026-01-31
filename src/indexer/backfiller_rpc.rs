use anyhow::Error;
use crate::{geyser::{SlotStatus, SubscribeUpdateSlot}, indexer::{db_processor::db_processor, processor_trait::SlotProcessor}, rpc::client::SolanaRpc};

pub struct Backfiller<'a, P>{
    rpc: &'a SolanaRpc,
    processor: &'a P
}

impl<'a, P> Backfiller<'a, P>
where P: SlotProcessor + Sync,{
    pub fn new(rpc: &'a SolanaRpc, processor: &'a P) -> Self {
        Self { rpc, processor }
    }
    
    pub async fn backfiller_range(
        &self,
        from_Slot: u64,
        to_Slot: u64,
        db_processor: &db_processor
    ) -> Result<(), Error>{
        for slot in (from_Slot + 1) ..= to_Slot {
            let block = self.rpc.get_finalized_block(slot).await?;

            let slot_meta = SubscribeUpdateSlot {
                slot,
                parent: Some(block.parentSlot),
                status: SlotStatus::SlotFinalized.into(),
                dead_error: Some(String::from("Null"))
            };

            self.processor.process_slot(slot_meta, self.rpc, db_processor).await?;
        }
        Ok(())
    }
}