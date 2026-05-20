use crate::message::Address;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub from: Address,
    pub to: Address,
    pub body: String,
}
