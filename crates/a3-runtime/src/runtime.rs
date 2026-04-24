//!

use crate::builder::RuntimeBuilder;
use a3_manifest::{Manifest, Provider};
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{anthropic, deepseek, gemini, ollama, openai, openrouter, xai},
};

///
pub struct Runtime {
    manifest: Manifest,
}

impl From<Manifest> for Runtime {
    fn from(manifest: Manifest) -> Self {
        Runtime { manifest }
    }
}

impl Runtime {
    ///
    pub async fn serve(self) -> crate::Result<()> {
        match self.manifest.provider {
            _ => {
                let builder = RuntimeBuilder::from(
                    openai::Client::from_env().agent(self.manifest.model.clone()),
                );
                builder.build();
            }
        };
        builder.name(self.manifest.name);
        if let Some(desctiption) = self.manifest.description {
            builder.description(&desctiption);
        }
        if let Some(instruction) = self.manifest.instruction {
            builder.preamble(&instruction);
        }
        Ok(())
    }
}

// async fn prepare_openai(manifest: Manifest) {}
