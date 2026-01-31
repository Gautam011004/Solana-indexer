use anyhow::{Error, Ok};
use sqlx::{PgPool, Row};

use crate::{geyser::{SlotStatus, SubscribeUpdateBlock, SubscribeUpdateSlot}, solana::storage::confirmed_block::ConfirmedBlock, types::SlotMeta};

#[derive(Clone)]
pub struct PostgresStorage {
    pool: PgPool
}

impl PostgresStorage {
    pub async fn new(database_url: &str) -> Result<Self, Error>{
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_checkpoint(&self, key: &str) -> Result<Option<u64>, Error>{
        let row = sqlx::query(
            "SELECT value from checkpoints where key = $1",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.get::<i64,_>("value") as u64))    
        
    } 

    pub async fn set_checkpoint(&self, key: &str, value: u64) -> Result<(), Error>{
        sqlx::query(
            r#"Insert into checkpoints where (key, value)
                    values ($1, $2)
                    on conflict (key)
                    do Update set value = Excluded.value
                    "#,
        )
        .bind(key)
        .bind(value as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_slot(&self, slot: &SubscribeUpdateSlot) -> Result<(),Error>{
        sqlx::query(
            r#"Insert into slots (slot, parent, status)
                    Values ($1, $2, $3) 
                    on conflict (slot) 
                    do update set 
                        parent = Excluded.parent
                        status = Excluded.status
                    "#
        )
        .bind(slot.slot as i64)
        .bind(slot.parent.map(|p| p as i64))
        .bind(slot_status_str(&SlotStatus::from_i32(slot.status).unwrap()))
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn insert_block(&self, block: &SubscribeUpdateBlock) -> Result<(), Error>{
        sqlx::query(
            r#"Insert into blocks (slot, blockhash, update_account_count, entries_count)
                    Values ($1, $2, $3, $4) on conflict (slot) 
                    do update set 
                        blockhash = Excluded.blockhash
                        update_account_count = Excluded.update_account_count
                        entries_count = Excluded.entries_count
                        status = Excluded.status
                    "#
        )
        .bind(block.slot as i64)
        .bind(&block.blockhash)
        .bind(block.updated_account_count as i64)
        .bind(block.entries_count as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

fn slot_status_str(status: &SlotStatus) -> &'static str{
    match status {
        SlotStatus::SlotConfirmed => "Confirmed",
        SlotStatus::SlotFinalized => "Finalized",
        SlotStatus::SlotProcessed => "Processed",
        SlotStatus::SlotCompleted => "Completed",
        SlotStatus::SlotDead => "Dead",
        SlotStatus::SlotCreatedBank => "Created Bank",
        SlotStatus::SlotFirstShredReceived => "First shred recieved"
    }
}