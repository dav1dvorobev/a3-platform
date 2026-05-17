use crate::message::Message;
use a3_manifest::Address;
use async_trait::async_trait;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct Sender {
    address: Address,
    client: async_nats::Client,
}

pub struct Receiver {
    subscription: async_nats::Subscriber,
}

pub async fn connect(address: Address) -> crate::Result<(Sender, Receiver)> {
    let client = async_nats::connect(std::env::var("NATS_URL")?).await?;
    let subscription = client.subscribe(subject_for_address(&address)).await?;
    Ok((Sender { address, client }, Receiver { subscription }))
}

#[async_trait]
impl crate::Sender for Sender {
    async fn send(&self, to: Address, body: String) -> crate::Result<()> {
        let subject = subject_for_address(&to);
        let message = Message {
            from: self.address.clone(),
            to,
            body,
        };
        let payload = serde_json::to_vec(&message)?;
        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }
}

#[async_trait]
impl crate::Receiver for Receiver {
    async fn recv(&mut self) -> crate::Result<Option<Message>> {
        let Some(message) = self.subscription.next().await else {
            return Ok(None);
        };
        let message = serde_json::from_slice::<Message>(&message.payload)?;
        Ok(Some(message))
    }
}

fn subject_for_address(address: &Address) -> String {
    format!(
        "{}.{}.{}",
        address.top_level_domain, address.second_level_domain, address.name
    )
}
