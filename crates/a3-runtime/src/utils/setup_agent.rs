use rig::{
    agent::Agent,
    completion::{Chat, CompletionModel},
};

pub async fn setup_agent<M>(
    agent: Agent<M>,
    sender: impl a3_transport::Sender,
    mut receiver: impl a3_transport::Receiver,
) -> crate::Result<()>
where
    M: CompletionModel + 'static,
{
    let mut chat_history = vec![];
    loop {
        match receiver.recv().await {
            Ok(message) => match message {
                Some(message) => {
                    tracing::info!("\n{message}");
                    match agent.chat(message.to_string(), &mut chat_history).await {
                        Ok(response) => {
                            tracing::info!("\n{response}");
                        }
                        Err(e) => {
                            tracing::error!("failed to process message: {e}");
                            let recovery_prompt = format!(
                                "<RECOVERY_PROMPT>\n\
                                The previous attempt to process message from {} failed.\n\
                                <ERROR>{e}</ERROR>\n\
                                You must notify the original sender by calling \"send_message\".\n\
                                Do not answer with normal text.\n\
                                After the tool call succeeds, return exactly:\n\
                                [ERROR SENT]\n\
                            </RECOVERY_PROMPT>",
                                message.from
                            );
                            match agent.chat(recovery_prompt, &mut chat_history).await {
                                Ok(response) if response.trim() == "[ERROR SENT]" => {}
                                Ok(response) => {
                                    sender.send(message.from.clone(), response).await?;
                                }
                                Err(e) => {
                                    sender.send(message.from.clone(), e.to_string()).await?;
                                    break;
                                }
                            }
                        }
                    }
                }
                None => {
                    tracing::warn!("transport receiver closed");
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
