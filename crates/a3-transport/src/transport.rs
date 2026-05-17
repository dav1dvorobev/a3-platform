use crate::message::Message;
use a3_manifest::Address;
use async_trait::async_trait;

#[async_trait]
pub trait Transport: Clone + Sized {
    async fn connect(manifest: Address) -> crate::Result<Self>
    where
        Self: Sized;

    async fn send(&self, to: Address, body: String) -> crate::Result<()>;

    async fn recv(&mut self) -> crate::Result<Option<Message>>;
}
