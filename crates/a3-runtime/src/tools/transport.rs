use a3_transport::{Sender, message::Address};
use rig::{completion::ToolDefinition, tool::Tool};
use std::str::FromStr;

pub struct Transport<T>
where
    T: Sender,
{
    sender: T,
}

impl<T> Transport<T>
where
    T: Sender,
{
    pub fn new(sender: T) -> Self {
        Self { sender }
    }
}

impl<T> Tool for Transport<T>
where
    T: Sender,
{
    const NAME: &'static str = "send_message";
    type Error = SendMessageToolError;
    type Args = serde_json::Value;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Send a message to another address.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "to": {
                        "type": "string",
                        "description": "Full recipient address. Use the FROM address to reply to the sender.
                        Use an address from the available agents list to delegate a task.
                        Example:\"user@email.local\"."
                    },
                    "body": {
                        "type": "string",
                        "description": "Message text delivered to the recipient. Include the task, question,
                        or final answer. If you need a reply, explicitly ask the recipient to send the result back."
                    }
                },
                "required": ["to", "body"],
                "additionalProperties": false
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let to = address_arg(&args, "to")?;
        let body = string_arg(&args, "body")?;
        self.sender
            .send(to, body)
            .await
            .map_err(SendMessageToolError::Transport)?;
        Ok("message sent successfully".to_string())
    }
}

fn address_arg(args: &serde_json::Value, field: &str) -> Result<Address, SendMessageToolError> {
    let value = string_arg(args, field)?;
    Address::from_str(value.as_str()).map_err(SendMessageToolError::InvalidAddress)
}

fn string_arg(args: &serde_json::Value, field: &str) -> Result<String, SendMessageToolError> {
    args.get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or(SendMessageToolError::InvalidArgument(field.to_string()))
}

#[derive(thiserror::Error, Debug)]
pub enum SendMessageToolError {
    #[error("invalid or missing \"{0}\" argument")]
    InvalidArgument(String),
    #[error("invalid address: {0}")]
    InvalidAddress(a3_transport::Error),
    #[error("failed to send message: {0}")]
    Transport(a3_transport::Error),
}
