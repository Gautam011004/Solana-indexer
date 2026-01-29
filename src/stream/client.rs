use anyhow::{Error, Ok};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Channel, ClientTlsConfig};

use crate::geyser::SubscribeRequest;
use crate::geyser::geyser_client::GeyserClient;
use crate::geyser::subscribe_update::UpdateOneof;

pub struct GrpcClient {
    inner : GeyserClient<Channel>
}

impl GrpcClient {
    pub async fn connect(endpoint : &str) -> Result<Self, Error>{
        let tls = ClientTlsConfig::new();
        let channel = Channel::from_shared(endpoint.to_string())?
                                .tls_config(tls)?
                                .connect()
                                .await?;
        let inner = GeyserClient::new(channel);
        Ok(Self { inner })
    }

    pub async fn subscribe(&mut self) -> Result<(), Error>{
        let (tx, rx) = mpsc::channel(1);
        tx.send(SubscribeRequest::default()).await?;
        let request_stream = ReceiverStream::new(rx);
        let mut stream = self.inner.subscribe(request_stream).await?.into_inner();
        println!("Connected to geyser client");
        while let Some(update) = stream.message().await? {
            match update.update_oneof {
                Some(UpdateOneof::Ping(_)) => {
                    println!("ping");
                }
        
                Some(UpdateOneof::Pong(_)) => {
                    println!("pong");
                }
        
                Some(UpdateOneof::Slot(slot)) => {
                    println!("slot {}", slot.slot);
                }
        
                Some(UpdateOneof::Block(block)) => {
                    println!("block at slot {}", block.slot);
                }
        
                Some(UpdateOneof::Transaction(tx)) => {
                    println!("transaction update {:?}", tx);
                }
        
                Some(UpdateOneof::Account(_)) => {
                    println!("account update");
                }
        
                Some(UpdateOneof::BlockMeta(meta)) => {
                    println!("block meta {}", meta.slot);
                }
        
                Some(UpdateOneof::Entry(_)) => {
                    println!("entry update");
                }
        
                Some(UpdateOneof::TransactionStatus(_)) => {
                    println!("tx status update");
                }
        
                None => {}
            }
        }
        Ok(())
    }

}