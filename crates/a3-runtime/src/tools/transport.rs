use a3_transport::{Sender, message::Address};
use rig::{completion::ToolDefinition, tool::Tool};
use std::{error::Error, fmt};

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
            description: "Send a message to another agent.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "to": {
                        "type": "string",
                        "description": "Address of the target agent"
                    },
                    "body": {
                        "type": "string",
                        "description": "Text payload to send to the target agent"
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
        Ok("message sent".to_string())
    }
}

fn address_arg(
    args: &serde_json::Value,
    field: &'static str,
) -> Result<Address, SendMessageToolError> {
    let value = string_arg(args, field)?;
    Address::from_str(value.as_str()).map_err(SendMessageToolError::InvalidAddress)
}

fn string_arg(
    args: &serde_json::Value,
    field: &'static str,
) -> Result<String, SendMessageToolError> {
    args.get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or(SendMessageToolError::InvalidArgument(field))
}

#[derive(Debug)]
pub enum SendMessageToolError {
    InvalidArgument(&'static str),
    InvalidAddress(a3_transport::Error),
    Transport(a3_transport::Error),
}

impl fmt::Display for SendMessageToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(field) => {
                write!(f, "invalid or missing `{field}` argument")
            }
            Self::InvalidAddress(e) => write!(f, "invalid address: {e}"),
            Self::Transport(e) => write!(f, "failed to send message: {e}"),
        }
    }
}

impl Error for SendMessageToolError {}
