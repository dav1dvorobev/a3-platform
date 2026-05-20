//! Transport types supported by manifests.

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Nats,
}
