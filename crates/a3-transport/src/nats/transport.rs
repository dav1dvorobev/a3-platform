use a3_manifest::Address;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;

use crate::message::Message;

#[derive(Clone)]
pub struct Transport {
    address: Arc<Address>,
    client: async_nats::Client,
    subscription: Arc<RwLock<async_nats::Subscriber>>,
}

#[async_trait]
impl crate::Transport for Transport {
    async fn connect(address: Address) -> crate::Result<Self> {
        let client = async_nats::connect(std::env::var("NATS_URL")?).await?;
        let subscription = Arc::new(RwLock::new(
            client
                .subscribe(format!(
                    "{}.{}.{}",
                    address.top_level_domain, address.second_level_domain, address.name
                ))
                .await?,
        ));
        Ok(Self {
            address: Arc::new(address),
            client,
            subscription,
        })
    }

    async fn send(&self, to: Address, body: String) -> crate::Result<()> {
        let subject = format!(
            "{}.{}.{}",
            to.top_level_domain, to.second_level_domain, to.name
        );
        let message = Message {
            from: self.address.as_ref().clone(),
            to,
            body,
        };
        let payload = serde_json::to_vec(&message)?;
        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }

    async fn recv(&mut self) -> crate::Result<Option<Message>> {
        let Some(message) = self.subscription.write().await.next().await else {
            return Ok(None);
        };
        let message = serde_json::from_slice::<Message>(&message.payload)?;
        Ok(Some(message))
    }
}
