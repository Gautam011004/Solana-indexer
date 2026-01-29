use anyhow::{Error, Ok};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;

use crate::geyser::SubscribeRequest;
use crate::geyser::geyser_client::GeyserClient;

pub struct GrpcClient {
    inner : GeyserClient<Channel>
}

impl GrpcClient {
    pub async fn connect(endpoint : &str) -> Result<Self, Error>{
        let inner = GeyserClient::connect(endpoint.to_string()).await?;
        Ok(Self { inner })
    }

    pub async fn subscribe(&mut self) -> Result<(), Error>{
        let (tx, rx) = mpsc::channel(1);
        tx.send(SubscribeRequest::default()).await?;
        let request_stream = ReceiverStream::new(rx);
        let mut stream = self.inner.subscribe(request_stream).await?.into_inner();
        println!("Connected to geyser client");
        while let Some(update) = stream.message().await? {
            println!("Update: {:?}", update);
        }
        Ok(())
    }

}