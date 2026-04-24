//! Manifest type and loading helpers.

use crate::{Provider, ToolDefinition};
use std::{collections::HashMap, path::Path};

/// Manifest definition.
#[derive(serde::Deserialize, Debug)]
#[non_exhaustive]
pub struct Manifest {
    pub name: String,
    pub provider: Provider,
    pub model: String,
    pub description: String,
    pub instruction: String,
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
        if self.name.trim().is_empty() {
            return Err(crate::Error::MissingField("name"));
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
            "name": "search",
            "provider": "openai",
            "model": "qwen2.5:1.5b",
            "description": "DuckDuckGo Searcher",
            "instruction": "Provide a concise summary results for topic using DuckDuckGo",
            "tools": {
                "duckduckgo": {
                    "type": "stdio",
                    "command": "docker",
                    "args": ["run", "-i", "--rm", "mcp/duckduckgo"]
                },
                "time": {
                    "type": "stdio",
                    "command": "docker",
                    "args": ["run", "-i", "--rm", "mcp/time"]
                }
            }
        }"#,
        )
        .unwrap();
        assert_eq!(manifest.name, "search");
        assert!(matches!(manifest.provider, Provider::OpenAI));
        assert_eq!(manifest.model, "qwen2.5:1.5b");
        assert_eq!(manifest.description, "DuckDuckGo Searcher");
        assert_eq!(
            manifest.instruction,
            "Provide a concise summary results for topic using DuckDuckGo"
        );
        let tools = manifest.tools.as_ref().unwrap();
        assert_eq!(tools.len(), 2);
        assert!(tools.contains_key("duckduckgo"));
        assert!(tools.contains_key("time"));
    }
}
