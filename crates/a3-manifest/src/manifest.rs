//!

use crate::{Provider, Tools};
use std::{collections::HashMap, path::Path};

///
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
    ///
    pub fn from_path(path: impl AsRef<Path>) -> crate::Result<Self> {
        let bytes = std::fs::read(path)?;
        let manifest = serde_json::from_slice::<Self>(&bytes)?;
        manifest.validate()?;
        Ok(manifest)
    }

    ///
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
mod tests {}
