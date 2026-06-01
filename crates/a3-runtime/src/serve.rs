use crate::{
    tools::Transport,
    utils::{build_agent, setup_agent, setup_tools},
};
use a3_manifest::{Manifest, Provider};
use rig::{
    client::ProviderClient,
    providers::{anthropic, deepseek, gemini, ollama, openai, openrouter, xai},
    tool::server::{ToolServer, ToolServerHandle},
};

// Hardcoded for the prototype.
const DEFAULT_MAX_TURNS: usize = 32;

/// Serve the agent with the supplied manifest.
pub async fn serve(manifest: &Manifest) -> crate::Result<()> {
    // Hardcoded for the prototype. Use transport from manifest.
    let (sender, receiver) = a3_transport::nats::connect(manifest.address.clone())
        .await
        .inspect_err(|e| tracing::error!("failed to connect transport: {e}"))?;
    let tool_server_handle = ToolServer::new().run();
    add_transport_tool(sender.clone(), &tool_server_handle)
        .await
        .inspect_err(|e| tracing::error!("failed to add transport tool: {e}"))?;
    let _services = setup_tools(manifest.tools.as_ref(), &tool_server_handle)
        .await
        .inspect_err(|e| tracing::error!("failed to setup tools: {e}"))?;
    let default_max_turns = DEFAULT_MAX_TURNS;
    match manifest.provider {
        Provider::Anthropic => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                anthropic::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::DeepSeek => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                deepseek::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::Gemini => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                gemini::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::Ollama => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                ollama::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::OpenAI => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                openai::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::OpenRouter => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                openrouter::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
        Provider::xAI => {
            tracing::info!("build agent \"{}\"", manifest.address);
            let agent = build_agent(
                xai::Client::from_env()?,
                manifest,
                tool_server_handle,
                default_max_turns,
            );
            tracing::info!("setup agent \"{}\"", manifest.address);
            setup_agent(agent, sender, receiver).await?;
        }
    }
    Ok(())
}

async fn add_transport_tool(
    sender: impl a3_transport::Sender,
    tool_server_handle: &ToolServerHandle,
) -> crate::Result<()> {
    tool_server_handle.add_tool(Transport::new(sender)).await?;
    Ok(())
}
