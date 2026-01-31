# Solana Blockchain Indexer

A high-performance Rust-based indexer for the Solana blockchain that streams real-time blockchain data via gRPC (Geyser) and stores it in PostgreSQL. The indexer supports both real-time streaming and historical backfilling of blockchain data.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Database Schema](#database-schema)
- [Customization Guide](#customization-guide)
  - [Understanding Proto Definitions](#understanding-proto-definitions)
  - [Customizing Data Storage](#customizing-data-storage)
  - [Adding New Data Types](#adding-new-data-types)
- [Usage](#usage)
- [Architecture](#architecture)
- [Development](#development)

## Overview

This indexer connects to a Solana Geyser gRPC endpoint to receive real-time updates about:
- **Slots**: Block slot information and status changes
- **Blocks**: Complete block data including transactions and accounts
- **Transactions**: Transaction details with metadata
- **Accounts**: Account state changes
- **Entries**: Entry-level blockchain data

The indexer automatically handles gaps in the blockchain by backfilling missing slots when detected, ensuring data consistency.

## Features

- Real-time blockchain data streaming via gRPC
- PostgreSQL storage backend
- Automatic gap detection and backfilling
- Checkpoint management for resumable indexing
- Support for multiple commitment levels (Processed, Confirmed, Finalized)
- Extensible processor architecture
- Type-safe protobuf definitions

## Project Structure

```
indexer/
├── proto/                          # Protocol Buffer definitions
│   ├── geyser.proto               # Geyser gRPC service definitions
│   └── solana-storage.proto       # Solana storage data structures
├── src/
│   ├── main.rs                    # Application entry point
│   ├── config.rs                  # Configuration management
│   ├── types.rs                   # Internal type definitions
│   ├── indexer/                   # Core indexing logic
│   │   ├── mod.rs
│   │   ├── processor_trait.rs     # Trait for processing slots/blocks
│   │   ├── db_processor.rs        # Database-backed processor implementation
│   │   └── backfiller_rpc.rs      # Historical data backfilling
│   ├── rpc/                       # Solana RPC client
│   │   ├── mod.rs
│   │   ├── client.rs              # RPC client implementation
│   │   └── types.rs               # RPC request/response types
│   ├── storage/                   # Storage abstractions
│   │   ├── mod.rs
│   │   └── postgres.rs            # PostgreSQL storage implementation
│   └── stream/                    # gRPC streaming client
│       ├── mod.rs
│       ├── client.rs              # Geyser gRPC client
│       └── geyser.rs              # Geyser-specific utilities
├── build.rs                       # Protobuf compilation build script
├── Cargo.toml                     # Rust dependencies and project config
└── README.md                      # This file
```

## Prerequisites

- **Rust** (Edition 2024) - [Install Rust](https://www.rust-lang.org/tools/install)
- **PostgreSQL** 12+ - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Solana Geyser gRPC endpoint** - Access to a Geyser-compatible endpoint
- **Solana RPC endpoint** - For backfilling historical data

## Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd indexer
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Set up PostgreSQL database:**
   ```sql
   CREATE DATABASE solana_indexer;
   
   -- Create tables
   CREATE TABLE checkpoints (
       key VARCHAR(255) PRIMARY KEY,
       value BIGINT NOT NULL
   );
   
   CREATE TABLE slots (
       slot BIGINT PRIMARY KEY,
       parent BIGINT,
       status VARCHAR(50) NOT NULL
   );
   
   CREATE TABLE blocks (
       slot BIGINT PRIMARY KEY,
       blockhash VARCHAR(64) NOT NULL,
       update_account_count BIGINT,
       entries_count BIGINT
   );
   ```

4. **Configure environment variables:**
   Create a `.env` file or set environment variables:
   ```bash
   DATABASE_URL=postgresql://user:password@localhost/solana_indexer
   GEYSER_ENDPOINT=https://your-geyser-endpoint:10000
   RPC_ENDPOINT=https://api.mainnet-beta.solana.com
   ```

## Configuration

The indexer requires the following configuration:

- **DATABASE_URL**: PostgreSQL connection string
- **GEYSER_ENDPOINT**: Geyser gRPC endpoint URL (e.g., `https://geyser.example.com:10000`)
- **RPC_ENDPOINT**: Solana JSON-RPC endpoint for backfilling

## Database Schema

The current schema includes:

### `checkpoints` Table
Stores checkpoint values for resumable indexing:
- `key` (VARCHAR): Checkpoint identifier (e.g., "last_finalized_slot")
- `value` (BIGINT): Checkpoint value (slot number)

### `slots` Table
Stores slot metadata:
- `slot` (BIGINT): Slot number (primary key)
- `parent` (BIGINT): Parent slot number (nullable)
- `status` (VARCHAR): Slot status (Processed, Confirmed, Finalized, etc.)

### `blocks` Table
Stores block information:
- `slot` (BIGINT): Slot number (primary key)
- `blockhash` (VARCHAR): Block hash
- `update_account_count` (BIGINT): Number of updated accounts
- `entries_count` (BIGINT): Number of entries

## Customization Guide

### Understanding Proto Definitions

The project uses Protocol Buffers to define data structures. The proto files are located in the `proto/` directory:

#### `proto/geyser.proto`
Defines the Geyser gRPC service and update messages:

**Key Messages:**
- `SubscribeUpdate`: Main update wrapper containing different update types
- `SubscribeUpdateSlot`: Slot status updates
- `SubscribeUpdateBlock`: Complete block data
- `SubscribeUpdateTransaction`: Transaction information
- `SubscribeUpdateAccount`: Account state changes
- `SubscribeUpdateEntry`: Entry-level data

**Example - SubscribeUpdateBlock:**
```protobuf
message SubscribeUpdateBlock {
  uint64 slot = 1;
  string blockhash = 2;
  solana.storage.confirmed_block.Rewards rewards = 3;
  solana.storage.confirmed_block.UnixTimestamp block_time = 4;
  solana.storage.confirmed_block.BlockHeight block_height = 5;
  uint64 parent_slot = 7;
  string parent_blockhash = 8;
  uint64 executed_transaction_count = 9;
  repeated SubscribeUpdateTransactionInfo transactions = 6;
  uint64 updated_account_count = 10;
  repeated SubscribeUpdateAccountInfo accounts = 11;
  uint64 entries_count = 12;
  repeated SubscribeUpdateEntry entries = 13;
}
```

#### `proto/solana-storage.proto`
Defines Solana-specific data structures:

**Key Messages:**
- `ConfirmedTransaction`: Transaction with metadata
- `Transaction`: Transaction structure with signatures and message
- `TransactionStatusMeta`: Transaction execution metadata
- `TokenBalance`: Token account balance information
- `Reward`: Staking/voting rewards
- `InnerInstructions`: Inner program instructions

**Example - TransactionStatusMeta:**
```protobuf
message TransactionStatusMeta {
    TransactionError err = 1;
    uint64 fee = 2;
    repeated uint64 pre_balances = 3;
    repeated uint64 post_balances = 4;
    repeated InnerInstructions inner_instructions = 5;
    repeated string log_messages = 6;
    repeated TokenBalance pre_token_balances = 7;
    repeated TokenBalance post_token_balances = 8;
    repeated Reward rewards = 9;
    optional uint64 compute_units_consumed = 16;
    optional uint64 cost_units = 17;
}
```

### Customizing Data Storage

To customize what data is stored, you need to:

1. **Modify the Database Schema**
   
   Add new tables or columns based on the proto definitions. For example, to store transaction data:

   ```sql
   CREATE TABLE transactions (
       signature VARCHAR(88) PRIMARY KEY,
       slot BIGINT NOT NULL,
       blockhash VARCHAR(64),
       fee BIGINT,
       err TEXT,
       compute_units_consumed BIGINT,
       created_at TIMESTAMP DEFAULT NOW(),
       FOREIGN KEY (slot) REFERENCES slots(slot)
   );
   
   CREATE TABLE transaction_accounts (
       transaction_signature VARCHAR(88),
       account_index INTEGER,
       pubkey VARCHAR(44) NOT NULL,
       writable BOOLEAN,
       signer BOOLEAN,
       pre_balance BIGINT,
       post_balance BIGINT,
       PRIMARY KEY (transaction_signature, account_index),
       FOREIGN KEY (transaction_signature) REFERENCES transactions(signature)
   );
   ```

2. **Update Storage Implementation**
   
   Modify `src/storage/postgres.rs` to add new insert methods:

   ```rust
   pub async fn insert_transaction(&self, tx: &SubscribeUpdateTransactionInfo) -> Result<(), Error> {
       sqlx::query(
           r#"INSERT INTO transactions (signature, slot, fee, err, compute_units_consumed)
              VALUES ($1, $2, $3, $4, $5)
              ON CONFLICT (signature) DO UPDATE SET
                  slot = EXCLUDED.slot,
                  fee = EXCLUDED.fee,
                  err = EXCLUDED.err
           "#
       )
       .bind(base58_encode(&tx.signature))
       .bind(tx.slot as i64)
       .bind(tx.meta.as_ref().map(|m| m.fee as i64))
       .bind(tx.meta.as_ref().and_then(|m| m.err.as_ref()).map(|e| format!("{:?}", e)))
       .bind(tx.meta.as_ref().and_then(|m| m.compute_units_consumed).map(|c| c as i64))
       .execute(&self.pool)
       .await?;
       Ok(())
   }
   ```

3. **Update Processor Implementation**
   
   Modify `src/indexer/db_processor.rs` to process and store new data types:

   ```rust
   #[async_trait]
   impl SlotProcessor for db_processor {
       async fn process_transaction(&self, tx: SubscribeUpdateTransaction) -> Result<(), Error> {
           if let Some(info) = tx.transaction {
               // Store transaction
               self.storage.insert_transaction(&info).await?;
               
               // Store transaction accounts if needed
               if let Some(meta) = info.meta {
                   // Process account balances, inner instructions, etc.
               }
           }
           Ok(())
       }
   }
   ```

4. **Update Stream Handler**
   
   Modify `src/stream/client.rs` to handle new update types:

   ```rust
   Some(UpdateOneof::Transaction(tx)) => {
       db_processor.process_transaction(tx).await?;
   }
   ```

### Adding New Data Types

To add support for new data types from the proto definitions:

1. **Identify the Proto Message**
   
   Review `proto/geyser.proto` and `proto/solana-storage.proto` to find the message type you want to store.

2. **Create Database Table**
   
   Design a table schema that captures the relevant fields from the proto message.

3. **Implement Storage Method**
   
   Add a method in `PostgresStorage` to insert the data.

4. **Implement Processor Method**
   
   Add processing logic in your processor implementation.

5. **Wire Up Stream Handler**
   
   Connect the stream update to your processor in `GrpcClient::subscribe`.

### Example: Storing Account Updates

Here's a complete example of adding account update storage:

**1. Database Schema:**
```sql
CREATE TABLE accounts (
    pubkey VARCHAR(44) PRIMARY KEY,
    slot BIGINT NOT NULL,
    owner VARCHAR(44),
    lamports BIGINT,
    executable BOOLEAN,
    rent_epoch BIGINT,
    data BYTEA,
    write_version BIGINT,
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_accounts_slot ON accounts(slot);
CREATE INDEX idx_accounts_owner ON accounts(owner);
```

**2. Storage Method:**
```rust
pub async fn insert_account(&self, account: &SubscribeUpdateAccountInfo, slot: u64) -> Result<(), Error> {
    sqlx::query(
        r#"INSERT INTO accounts (pubkey, slot, owner, lamports, executable, rent_epoch, data, write_version)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (pubkey) DO UPDATE SET
               slot = EXCLUDED.slot,
               owner = EXCLUDED.owner,
               lamports = EXCLUDED.lamports,
               executable = EXCLUDED.executable,
               rent_epoch = EXCLUDED.rent_epoch,
               data = EXCLUDED.data,
               write_version = EXCLUDED.write_version,
               updated_at = NOW()
        "#
    )
    .bind(base58_encode(&account.pubkey))
    .bind(slot as i64)
    .bind(base58_encode(&account.owner))
    .bind(account.lamports as i64)
    .bind(account.executable)
    .bind(account.rent_epoch as i64)
    .bind(&account.data)
    .bind(account.write_version as i64)
    .execute(&self.pool)
    .await?;
    Ok(())
}
```

**3. Processor Update:**
```rust
async fn process_account(&self, account: SubscribeUpdateAccount) -> Result<(), Error> {
    if let Some(info) = account.account {
        self.storage.insert_account(&info, account.slot).await?;
    }
    Ok(())
}
```

**4. Stream Handler:**
```rust
Some(UpdateOneof::Account(account)) => {
    db_processor.process_account(account).await?;
}
```

## Usage

### Running the Indexer

```bash
cargo run --release
```

The indexer will:
1. Connect to the Geyser gRPC endpoint
2. Subscribe to slot and block updates
3. Process and store data in PostgreSQL
4. Automatically backfill gaps when detected

### Monitoring

Check the database to monitor indexing progress:

```sql
-- Get latest indexed slot
SELECT value FROM checkpoints WHERE key = 'last_finalized_slot';

-- Get slot statistics
SELECT status, COUNT(*) 
FROM slots 
GROUP BY status;

-- Get recent blocks
SELECT slot, blockhash, updated_account_count, entries_count
FROM blocks
ORDER BY slot DESC
LIMIT 10;
```

## Architecture

### Components

1. **Stream Client** (`src/stream/client.rs`)
   - Manages gRPC connection to Geyser
   - Handles subscription and message streaming
   - Routes updates to processors

2. **RPC Client** (`src/rpc/client.rs`)
   - JSON-RPC client for Solana
   - Used for backfilling historical data
   - Fetches finalized blocks

3. **Database Processor** (`src/indexer/db_processor.rs`)
   - Implements `SlotProcessor` trait
   - Handles slot and block processing
   - Manages checkpoints and gap detection

4. **Backfiller** (`src/indexer/backfiller_rpc.rs`)
   - Fills gaps in indexed slots
   - Uses RPC client to fetch missing blocks
   - Ensures data continuity

5. **Storage** (`src/storage/postgres.rs`)
   - PostgreSQL operations
   - Checkpoint management
   - Data insertion with conflict handling

### Data Flow

```
Geyser gRPC → Stream Client → Processor → Storage → PostgreSQL
                                      ↓
                              Gap Detection 
                                      ↓
                                Backfiller 
                                      ↓
                                RPC Client → Processor → Storage → PostgreSQL
``` 
→  → 
## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Generation

The protobuf files are automatically compiled during build via `build.rs`. To regenerate manually:

```bash
cargo build
```

### Adding Dependencies

Edit `Cargo.toml` and add your dependency:

```toml
[dependencies]
your-crate = "1.0"
```