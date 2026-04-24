//!

use a3_manifest::{Manifest, Provider, ToolDefinition};
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{anthropic, deepseek, gemini, ollama, openai, openrouter, xai},
    tool::{
        rmcp::McpClientHandler,
        server::{ToolServer, ToolServerHandle},
    },
};
use rmcp::{
    model::{ClientCapabilities, ClientInfo, Implementation},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use tokio::process::Command;

///
pub async fn serve(manifest: Manifest) -> crate::Result<()> {
    let tool_server_handle = ToolServer::new().run();
    let client_info = ClientInfo::new(
        ClientCapabilities::default(),
        Implementation::from_build_env(),
    );
    let mut services = Vec::new();
    if let Some(tools) = manifest.tools {
        for (name, description) in tools {
            let handler = McpClientHandler::new(client_info.clone(), tool_server_handle.clone());
            let service = match description {
                ToolDefinition::Http { url, headers } => {
                    let transport = StreamableHttpClientTransport::from_config(
                        StreamableHttpClientTransportConfig::with_uri(url),
                    );
                    handler.connect(transport).await?
                }
                ToolDefinition::Stdio { command, args, env } => {
                    let mut command = Command::new(command);
                    if let Some(args) = args {
                        command.args(args);
                    }
                    if let Some(env) = env {
                        command.envs(env);
                    }
                    let transport = rmcp::transport::TokioChildProcess::new(command)?;
                    handler.connect(transport).await?
                }
            };
            services.push(service);
        }
    }
    match manifest.provider {
        _ => {
            let agent = openai::Client::from_env()
                .agent(manifest.model.as_str())
                .name(manifest.name.as_str())
                .description(manifest.description.as_str())
                .preamble(manifest.instruction.as_str())
                .tool_server_handle(tool_server_handle)
                .build();
        }
    };
    Ok(())
}

///
async fn setup_tools() -> crate::Result<()> {
    Ok(())
}
