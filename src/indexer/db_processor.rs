use anyhow::{Error, Ok, anyhow};
use tokio::sync::Mutex;
use tonic::async_trait;
use crate::{geyser::{SlotStatus, SubscribeUpdate, SubscribeUpdateSlot}, indexer::{processor_trait::SlotProcessor, backfiller_rpc::{self, Backfiller}}, rpc::client::SolanaRpc, solana::storage::confirmed_block::ConfirmedBlock, storage::postgres::PostgresStorage, types::SlotMeta};

const FINALIZED_CHECKPOINT: &str = "last_finalized_slot";

pub struct db_processor {
    pub storage: PostgresStorage,
    pub last_finalized: Mutex<Option<u64>>
}

impl db_processor {
    pub async fn new(storage: PostgresStorage) -> Result<Self, Error> {
        let last_finalized = storage.get_checkpoint(FINALIZED_CHECKPOINT).await?;

        Ok(Self {storage , last_finalized: Mutex::new(last_finalized)})
    }
}


#[async_trait]
impl SlotProcessor for db_processor {
    async fn process_slot(&self, slot: SubscribeUpdateSlot, rpc: &SolanaRpc, processor: &db_processor) -> Result<(), Error>{
        self.storage.insert_slot(&slot).await?;

        if SlotStatus::from_i32(slot.status).unwrap() == SlotStatus::SlotFinalized {
            let mut last = self.last_finalized.lock().await;

            if let Some(prev) = *last {
                if prev + 1 != slot.slot {
                    let backfiller = Backfiller::new(&rpc, processor);
                    backfiller.backfiller_range(prev, slot.slot, processor).await?;
                }
            }
            self.storage.set_checkpoint(FINALIZED_CHECKPOINT, slot.slot).await?;

            *last = Some(slot.slot);
            
            println!("Processed slot {}", slot.slot);
        }
        Ok(())
    }
}
