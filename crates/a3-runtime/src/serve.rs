//!

use a3_manifest::{Manifest, Provider, ToolDefinition};
use http::{HeaderName, HeaderValue};
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, Prompt},
    providers::{anthropic, deepseek, gemini, ollama, openai, openrouter, xai},
    tool::{
        rmcp::McpClientHandler,
        server::{ToolServer, ToolServerHandle},
    },
};
use rmcp::{
    model::{ClientCapabilities, ClientInfo, Implementation},
    service::RunningService,
    transport::{
        StreamableHttpClientTransport, TokioChildProcess,
        streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use std::collections::HashMap;
use tokio::process::Command;

type McpService = RunningService<rmcp::service::RoleClient, McpClientHandler>;

///
pub async fn serve(manifest: Manifest) -> crate::Result<()> {
    let tool_server_handle = ToolServer::new().run();
    let _tool_services = setup_tools(manifest.tools.as_ref(), &tool_server_handle)
        .await
        .inspect_err(|e| tracing::error!("failed to setup tools: {e}"))?;
    match manifest.provider {
        Provider::Anthropic => {
            let agent = build_agent(anthropic::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::DeepSeek => {
            let agent = build_agent(deepseek::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::Gemini => {
            let agent = build_agent(gemini::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::Ollama => {
            let agent = build_agent(ollama::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::OpenAI => {
            let agent = build_agent(openai::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::OpenRouter => {
            let agent = build_agent(openrouter::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
        Provider::xAI => {
            let agent = build_agent(xai::Client::from_env(), manifest, tool_server_handle);
            setup_agent(agent).await?;
        }
    }
    Ok(())
}

async fn setup_agent<M>(agent: Agent<M>) -> crate::Result<()>
where
    M: CompletionModel + 'static,
{
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn setup_tools(
    tools: Option<&HashMap<String, ToolDefinition>>,
    tool_server_handle: &ToolServerHandle,
) -> crate::Result<Option<Vec<McpService>>> {
    let Some(tools) = tools else {
        return Ok(None);
    };
    let client_info = ClientInfo::new(
        ClientCapabilities::default(),
        Implementation::from_build_env(),
    );
    let mut tool_services = Vec::with_capacity(tools.len());
    for (name, definition) in tools {
        tracing::info!(tool = %name, "connecting manifest tool");
        let service = connect_tool(definition, client_info.clone(), tool_server_handle.clone())
            .await
            .inspect_err(|e| tracing::error!(tool = %name, "failed to connect tool: {e}"))?;
        tool_services.push(service);
    }
    Ok(Some(tool_services))
}

async fn connect_tool(
    definition: &ToolDefinition,
    client_info: ClientInfo,
    tool_server_handle: ToolServerHandle,
) -> crate::Result<McpService> {
    let handler = McpClientHandler::new(client_info, tool_server_handle);
    match definition {
        ToolDefinition::Http { url, headers } => {
            let config = match headers {
                Some(headers) => StreamableHttpClientTransportConfig::with_uri(url.as_str())
                    .custom_headers(parse_headers(headers)?),
                None => StreamableHttpClientTransportConfig::with_uri(url.as_str()),
            };
            let transport = StreamableHttpClientTransport::from_config(config);
            Ok(handler.connect(transport).await?)
        }
        ToolDefinition::Stdio { command, args, env } => {
            let mut command = Command::new(command);
            if let Some(args) = args {
                command.args(args);
            }
            if let Some(env) = env {
                command.envs(env);
            }
            let transport = TokioChildProcess::new(command)?;
            Ok(handler.connect(transport).await?)
        }
    }
}

fn parse_headers(
    headers: &HashMap<String, String>,
) -> crate::Result<HashMap<HeaderName, HeaderValue>> {
    headers
        .iter()
        .map(|(name, value)| {
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .inspect_err(|e| tracing::error!("invalid HTTP header name `{}`: {}", name, e))?;
            let header_value = HeaderValue::from_str(&value).inspect_err(|e| {
                tracing::error!("invalid HTTP header value for `{}`: {}", name, e)
            })?;
            Ok((header_name, header_value))
        })
        .collect()
}

fn build_agent<C>(
    client: C,
    manifest: Manifest,
    tool_server_handle: ToolServerHandle,
) -> Agent<C::CompletionModel>
where
    C: CompletionClient,
{
    client
        .agent(manifest.model.as_str())
        .name(manifest.name.as_str())
        .description(manifest.description.as_str())
        .preamble(manifest.instruction.as_str())
        .tool_server_handle(tool_server_handle)
        .build()
}
