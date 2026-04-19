//! Manifest type and loading helpers.

use crate::{Provider, Tools};
use std::{collections::HashMap, path::Path};

/// A manifest definition.
#[derive(serde::Deserialize, Debug)]
#[non_exhaustive]
pub struct Manifest {
    pub name: String,
    pub provider: Provider,
    pub model: String,
    pub env: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub instruction: Option<String>,
    pub tools: Option<Tools>,
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Manifest;
    use crate::Provider;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn deserializes_manifest_correctly() {
        let manifest: Manifest = serde_json::from_str(
            r#"{
            "name": "search",
            "provider": "openai",
            "model": "qwen2.5:1.5b",
            "env": {
                "OPENAI_BASE_URL": "http://localhost:11434/v1",
                "OPENAI_API_KEY": "ollama"
            },
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
        assert_eq!(
            manifest
                .env
                .as_ref()
                .unwrap()
                .get("OPENAI_API_KEY")
                .map(String::as_str),
            Some("ollama")
        );
        assert_eq!(
            manifest.instruction.as_deref(),
            Some("Provide a concise summary results for topic using DuckDuckGo")
        );
        let tools = manifest.tools.as_ref().unwrap();
        assert_eq!(tools.len(), 2);
        assert!(tools.contains_key("duckduckgo"));
        assert!(tools.contains_key("time"));
    }

    #[test]
    fn loads_manifest_from_path_correctly() {
        let path = unique_test_file_path();
        fs::write(
            &path,
            r#"{
                "name": "search",
                "provider": "ollama",
                "model": "qwen2.5:1.5b"
            }"#,
        )
        .unwrap();
        let manifest = Manifest::from_path(&path).unwrap();
        assert_eq!(manifest.name, "search");
        assert!(matches!(manifest.provider, Provider::Ollama));
        assert_eq!(manifest.model, "qwen2.5:1.5b");
        fs::remove_file(path).unwrap();
    }

    fn unique_test_file_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("a3-manifest-{nanos}.json"))
    }
}
