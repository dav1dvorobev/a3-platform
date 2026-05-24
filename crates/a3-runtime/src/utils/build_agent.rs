use a3_manifest::Manifest;
use rig::{agent::Agent, client::CompletionClient, tool::server::ToolServerHandle};

pub fn build_agent<C>(
    client: C,
    manifest: &Manifest,
    tool_server_handle: ToolServerHandle,
    default_max_turns: usize,
) -> Agent<C::CompletionModel>
where
    C: CompletionClient,
{
    let preamble = format!(
        "<PRELUDE>\n\
        You are an addressable, autonomous, isolated node.\n\
        You interact with other nodes only by sending and receiving messages.\n\
        \n\
        DEFINITIONS:\n\
        - \"public_chat\" - normal returned response.\n\
        - \"payload\" - answers, tools result, task details, delegated results, analysis, message contents or other useful content.\n\
        - tool \"send_message\" - this is the only place where \"payload\" may be sent.
        \n\
        NEVER write \"payload\" in \"public_chat\"\n\
        YOU MUST write in \"public_chat\" only a brief status summary of what was done \n\
        \n\
        If you need to reply to another node:\n\
        1. Prepare the final answer internally.\n\
        2. Send the full final answer only by calling tool \"send_message\"\n\
        </PRELUDE>\n
        YOUR ADDRESS:\n\
        {}\n\n\
        DESCRIPTION:\n\
        {}\n\n\
        INSTRUCTION:\n\
        {}\n\n\
        CONSTRAINTS:\n\
        {}",
        manifest.address, manifest.description, manifest.instruction, manifest.constraints
    );
    client
        .agent(manifest.model.as_str())
        .name(manifest.address.name.as_str())
        .preamble(preamble.as_str())
        .context(manifest.context.as_str())
        .default_max_turns(default_max_turns)
        .tool_server_handle(tool_server_handle)
        .build()
}
