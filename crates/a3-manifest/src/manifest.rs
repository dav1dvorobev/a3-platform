//!

use crate::{Provider, Tools};
use std::collections::HashMap;

///
#[derive(serde::Deserialize, Debug)]
#[non_exhaustive]
pub struct Manifest {
    pub provider: Provider,
    pub name: String,
    pub model: String,
    pub env: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub instruction: Option<String>,
    pub tools: Option<Tools>,
}

#[cfg(test)]
mod tests {}
