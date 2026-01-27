use anyhow::{Error, Ok, anyhow};
use tokio::sync::Mutex;
use tonic::async_trait;
use crate::{indexer::backfill::SlotProcessor, storage::postgres::PostgresStorage, types::{SlotMeta, SlotStatus}};

const FINALIZED_CHECKPOINT: &str = "last_finalized_slot";

pub struct db_processor {
    storage: PostgresStorage,
    last_finalized: Mutex<Option<u64>>
}

impl db_processor {
    pub async fn new(storage: PostgresStorage) -> Result<Self, Error> {
        let last_finalized = storage.get_checkpoint(FINALIZED_CHECKPOINT).await?;

        Ok(Self {storage , last_finalized: Mutex::new(last_finalized)})
    }
}


#[async_trait]
impl SlotProcessor for db_processor {
    async fn process_slot(&self, slot: SlotMeta) -> Result<(), Error>{
        self.storage.insert_slot(&slot).await?;

        if slot.status == SlotStatus::Finalized {
            let mut last = self.last_finalized.lock().await;

            if let Some(prev) = *last {
                if prev + 1 != slot.slot {
                    return Err(anyhow!("Error gap detected, expected slot {} got slot {}", prev + 1, slot.slot));
                }
            }

            self.storage.set_checkpoint(FINALIZED_CHECKPOINT, slot.slot).await?;

            *last = Some(slot.slot);
            
            println!("Processed slot {}", slot.slot);
        }
        Ok(())
    }
}
