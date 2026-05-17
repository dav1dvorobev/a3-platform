//!

use a3_manifest::{Manifest, Provider, ToolDefinition};
use http::{HeaderName, HeaderValue};
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    completion::{Chat, CompletionModel},
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
pub async fn serve(manifest: &Manifest) -> crate::Result<()> {
    // TODO: NATS hardcoded for the prototype. Move transport selection to manifest.
    let (sender, receiver) = a3_transport::nats::connect(manifest.address.clone())
        .await
        .inspect_err(|e| tracing::error!("failed to connect transport: {e}"))?;
    let tool_server_handle = ToolServer::new().run();
    add_transport_tool(sender, &tool_server_handle)
        .await
        .inspect_err(|e| tracing::error!("failed to add transport tool: {e}"))?;
    let services = setup_tools(manifest.tools.as_ref(), &tool_server_handle)
        .await
        .inspect_err(|e| tracing::error!("failed to setup tools: {e}"))?;
    let default_max_turns = services.as_ref().map_or(1, |services| services.len() + 1);
    match manifest.provider {
        Provider::Anthropic => {
            let agent = build_agent(
                anthropic::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::DeepSeek => {
            let agent = build_agent(
                deepseek::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::Gemini => {
            let agent = build_agent(
                gemini::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::Ollama => {
            let agent = build_agent(
                ollama::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::OpenAI => {
            let agent = build_agent(
                openai::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::OpenRouter => {
            let agent = build_agent(
                openrouter::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
        Provider::xAI => {
            let agent = build_agent(
                xai::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            setup_agent(agent, receiver).await?;
        }
    }
    Ok(())
}

async fn add_transport_tool<S>(
    sender: S,
    tool_server_handle: &ToolServerHandle,
) -> crate::Result<()>
where
    S: a3_transport::Sender,
{
    tool_server_handle
        .add_tool(crate::tools::Transport::new(sender))
        .await?;
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
    let mut services = Vec::with_capacity(tools.len());
    for (name, definition) in tools {
        let service = connect_tool(definition, client_info.clone(), tool_server_handle.clone())
            .await
            .inspect_err(|e| tracing::error!("failed to connect tool \"{name}\": {e}"))?;
        services.push(service);
        tracing::info!("connected tool \"{name}\"");
    }
    Ok(Some(services))
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
    manifest: &Manifest,
    tool_server_handle: ToolServerHandle,
    default_max_turns: usize,
) -> Agent<C::CompletionModel>
where
    C: CompletionClient,
{
    let preamble = format!(
        "DESCRIPTION:\n{}\n\nINSTRUCTION:\n{}\n\nCONSTRAINTS:\n{}",
        manifest.description.as_str(),
        manifest.instruction.as_str(),
        manifest.constraints.as_str()
    );
    client
        .agent(manifest.model.as_str())
        .preamble(preamble.as_str())
        .context(manifest.context.as_str())
        .default_max_turns(default_max_turns)
        .tool_server_handle(tool_server_handle)
        .build()
}

async fn setup_agent<M>(
    agent: Agent<M>,
    mut receiver: impl a3_transport::Receiver,
) -> crate::Result<()>
where
    M: CompletionModel + 'static,
{
    let mut history = vec![];
    loop {
        match receiver.recv().await {
            Ok(message) => match message {
                Some(message) => {
                    tracing::info!("received message: {message:?}");
                    let _ = agent.chat(message.to_string()?, &mut history).await?;
                }
                None => {
                    tracing::error!("failed to receive message");
                    break;
                }
            },
            Err(e) => {
                tracing::error!("failed to receive message: {e}");
                break;
            }
        }
    }
    Ok(())
}
