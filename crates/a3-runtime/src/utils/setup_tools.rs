use a3_manifest::ToolDefinition;
use http::{HeaderName, HeaderValue};
use rig::tool::{rmcp::McpClientHandler, server::ToolServerHandle};
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

pub async fn setup_tools(
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
                command.envs(
                    env.iter()
                        .map(|(key, value)| (key.as_str(), value.as_str())),
                );
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
                .inspect_err(|e| tracing::error!("invalid HTTP header name \"{}\": {}", name, e))?;
            let header_value = HeaderValue::from_str(&value).inspect_err(|e| {
                tracing::error!("invalid HTTP header value for \"{}\": {}", name, e)
            })?;
            Ok((header_name, header_value))
        })
        .collect()
}
