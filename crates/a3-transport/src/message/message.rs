use a3_manifest::Address;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub from: Address,
    pub to: Address,
    pub body: String,
}

impl Message {
    pub fn to_string(&self) -> crate::Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}
