#[derive(Debug,Clone)]
pub struct SlotMeta {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: SlotStatus
}

#[derive(Debug,Clone,PartialEq)]
pub enum SlotStatus{
    Processed,
    Confirmed,
    Finalized
}

#[derive(Debug,Clone)]
pub struct IndexedTransactions {
    pub txnsign: u64,
    pub slot: u64,
    pub status: SlotStatus
}