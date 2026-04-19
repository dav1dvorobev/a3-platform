//!

use std::collections::HashMap;

///
pub type Tools = Vec<(String, ToolDefinition)>;

///
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
mod tests {}
