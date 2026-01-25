use anyhow::Error;

use crate::{indexer::backfill::SlotProcessor, rpc::client::SolanaRpc, types::SlotMeta};

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
        to_Slot: u64
    ) -> Result<(), Error>{
        for slot in (from_Slot + 1) ..= to_Slot {
            let block = self.rpc.get_finalized_block(slot).await?;

            let slot_meta = SlotMeta {
                slot,
                parent: Some(block.parentSlot),
                status: crate::types::SlotStatus::Finalized
            };

            self.processor.process_slot(slot_meta).await?;
        }
        Ok(())
    }
}