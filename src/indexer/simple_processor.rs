use anyhow::{Error, Ok, anyhow};
use tokio::sync::Mutex;
use tonic::async_trait;

use crate::{indexer::backfill::SlotProcessor, types::{SlotMeta, SlotStatus}};

pub struct Simpleprocessor{
    last_finalized_slot: Mutex<Option<u64>>
}

impl Simpleprocessor {
    pub fn new() -> Self{
        Self { last_finalized_slot: Mutex::new(None),
        }
    }
}

#[async_trait]
impl SlotProcessor for Simpleprocessor{
    async fn process_slot(&self, slot: SlotMeta) -> Result<(), Error>{

        if slot.status != SlotStatus::Finalized {
            return Err(anyhow!("Simple processor only processes finalized slots"));
        }
        
        let mut last = self.last_finalized_slot.lock().await;
        
        if let Some(prev) = *last {
            if slot.slot != prev + 1 {
                return Err(anyhow!("gap detected"));
            }
        }

        println!("Indexed finalized slot {:?}", slot);

        *last = Some(slot.slot);
        Ok(())
    }
}