use crate::{Provider, ToolDefinition};
use std::{collections::HashMap, path::Path};

/// Manifest definition.
#[derive(serde::Deserialize, Debug)]
#[non_exhaustive]
pub struct Manifest {
    pub address: String,
    pub provider: Provider,
    pub model: String,
    pub description: String,
    pub instruction: String,
    pub constraints: String,
    pub context: String,
    pub tools: Option<HashMap<String, ToolDefinition>>,
}

impl Manifest {
    /// Loads, deserializes, and validates manifest from a JSON file.
    pub fn from_path(path: impl AsRef<Path>) -> crate::Result<Self> {
        let bytes = std::fs::read(path)?;
        let manifest = serde_json::from_slice::<Self>(&bytes)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validates required manifest fields.
    fn validate(&self) -> crate::Result<()> {
        if self.address.trim().is_empty() {
            return Err(crate::Error::MissingField("address"));
        }
        if self.model.trim().is_empty() {
            return Err(crate::Error::MissingField("model"));
        }
        if self.description.trim().is_empty() {
            return Err(crate::Error::MissingField("description"));
        }
        if self.instruction.trim().is_empty() {
            return Err(crate::Error::MissingField("instruction"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_manifest_correctly() {
        let manifest: Manifest = serde_json::from_str(
            r#"{
            "address": "search@email.local",
            "provider": "openai",
            "model": "Nemotron-3-Nano-4B-RotorQuant-MLX-4bit",
            "description": "Agent that searches external sources and summarizes results",
            "instruction": "Use available search tools. Return concise, sourced answers",
            "constraints": "If information is missing or access is limited, say so clearly instead of guessing",
            "context": "",
            "tools": {
                "duckduckgo": {
                    "type": "stdio",
                    "command": "docker",
                    "args": ["run", "-i", "--rm", "mcp/duckduckgo"]
                }
            }
        }"#,
        )
        .unwrap();
        assert_eq!(manifest.address, "search@email.local");
        assert!(matches!(manifest.provider, Provider::OpenAI));
        assert_eq!(manifest.model, "Nemotron-3-Nano-4B-RotorQuant-MLX-4bit");
        assert_eq!(
            manifest.description,
            "Agent that searches external sources and summarizes results"
        );
        assert_eq!(
            manifest.instruction,
            "Use available search tools. Return concise, sourced answers"
        );
        assert_eq!(
            manifest.constraints,
            "If information is missing or access is limited, say so clearly instead of guessing"
        );
        assert_eq!(manifest.context, "");
        let tools = manifest.tools.as_ref().unwrap();
        assert_eq!(tools.len(), 1);
        assert!(tools.contains_key("duckduckgo"));
    }
}
