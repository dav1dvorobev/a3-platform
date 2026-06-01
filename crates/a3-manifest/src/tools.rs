//! Tool definitions used by manifest.

use std::collections::HashMap;

pub struct EnvValue(String);

impl EnvValue {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'de> serde::Deserialize<'de> for EnvValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = <String as serde::Deserialize>::deserialize(deserializer)?;
        let expanded = shellexpand::env(&raw)
            .map_err(serde::de::Error::custom)?
            .into_owned();
        Ok(Self(expanded))
    }
}

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolDefinition {
    Http {
        url: String,
        headers: Option<HashMap<String, String>>,
    },
    Stdio {
        command: String,
        args: Option<Vec<String>>,
        env: Option<HashMap<String, EnvValue>>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_http_tool_definition_correctly() {
        let definition: ToolDefinition = serde_json::from_str(
            r#"{
                "type": "http",
                "url": "https://example.com",
                "headers": {
                    "Authorization": "Bearer <token>"
                }
            }"#,
        )
        .unwrap();
        match definition {
            ToolDefinition::Http { url, headers } => {
                assert_eq!(url, "https://example.com");
                assert_eq!(
                    headers.unwrap().get("Authorization").map(String::as_str),
                    Some("Bearer <token>")
                );
            }
            ToolDefinition::Stdio { .. } => panic!("expected http tool definition"),
        }
    }

    #[test]
    fn deserializes_stdio_tool_definition_correctly() {
        let definition: ToolDefinition = serde_json::from_str(
            r#"{
                "type": "stdio",
                "command": "docker",
                "args": ["run", "-i", "--rm", "mcp/example"],
                "env": {
                    "ACCESS_TOKEN": "TOKEN"
                }
            }"#,
        )
        .unwrap();
        match definition {
            ToolDefinition::Http { .. } => panic!("expected stdio tool definition"),
            ToolDefinition::Stdio { command, args, env } => {
                assert_eq!(command, "docker");
                assert_eq!(
                    args.unwrap(),
                    vec![
                        "run".to_string(),
                        "-i".to_string(),
                        "--rm".to_string(),
                        "mcp/example".to_string()
                    ]
                );
                assert_eq!(
                    env.unwrap().get("ACCESS_TOKEN").unwrap().as_str(),
                    EnvValue("TOKEN".to_string()).as_str()
                );
            }
        }
    }
}
