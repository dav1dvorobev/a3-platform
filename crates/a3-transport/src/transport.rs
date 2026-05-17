use crate::message::Message;
use a3_manifest::Address;
use async_trait::async_trait;

#[async_trait]
pub trait Sender: Clone + Send + Sync + 'static {
    async fn send(&self, to: Address, body: String) -> crate::Result<()>;
}

#[async_trait]
pub trait Receiver: Send {
    async fn recv(&mut self) -> crate::Result<Option<Message>>;
}
