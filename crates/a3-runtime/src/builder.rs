use rig::{
    agent::{Agent, AgentBuilder, WithBuilderTools},
    completion::CompletionModel,
};

pub enum RuntimeBuilder<M>
where
    M: CompletionModel,
{
    WithTools(AgentBuilder<M, (), WithBuilderTools>),
    WithoutTools(AgentBuilder<M>),
}

impl<M> RuntimeBuilder<M>
where
    M: CompletionModel,
{
    pub fn from(builder: AgentBuilder<M>) -> Self {
        Self::WithoutTools(builder)
    }

    ///
    pub fn build(self) -> Agent<M> {
        match self {
            Self::WithTools(builder) => builder.build(),
            Self::WithoutTools(builder) => builder.build(),
        }
    }
}
