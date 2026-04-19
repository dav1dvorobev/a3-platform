//! Tool definitions used by manifest.

use std::collections::HashMap;

/// Collection of tool definitions from the manifest.
///
/// Each entry is a `tool_name: tool_definition`.
pub type Tools = HashMap<String, ToolDefinition>;

/// A tool definition.
#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolDefinition {
    Http {
        url: String,
        headers: Option<HashMap<String, String>>,
    },
    Stdio {
        command: String,
        args: Option<Vec<String>>,
        env: Option<HashMap<String, String>>,
    },
}

#[cfg(test)]
mod tests {
    use super::ToolDefinition;

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
                    "ACCESS_TOKEN": "<TOKEN>"
                }
            }"#,
        )
        .unwrap();
        match definition {
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
                    env.unwrap().get("ACCESS_TOKEN").map(String::as_str),
                    Some("<TOKEN>")
                );
            }
            ToolDefinition::Http { .. } => panic!("expected stdio tool definition"),
        }
    }
}
