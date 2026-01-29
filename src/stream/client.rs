use anyhow::{Error, Ok};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Channel, ClientTlsConfig};

use crate::geyser::SubscribeRequest;
use crate::geyser::geyser_client::GeyserClient;

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
            println!("Update: {:?}", update);
        }
        Ok(())
    }

}