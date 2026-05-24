use crate::message::Address;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub from: Address,
    pub to: Address,
    pub body: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<MESSAGE>\n\
            <FROM>{}</FROM>\n\
            <BODY>{}</BODY>\n\
            </MESSAGE>",
            self.from, self.body
        )
    }
}
