//! Provider types supported by manifests.

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Provider {
    Anthropic,
    DeepSeek,
    Gemini,
    Ollama,
    OpenAI,
    OpenRouter,
    xAI,
}
