use anyhow::Result;
use tonic::async_trait;

use crate::types::SlotMeta;

#[async_trait]
pub trait SlotProcessor{
    async fn process_slot(&self, slot: SlotMeta) -> Result<()>;
}