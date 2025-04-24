use super::{
    gen_config_builder::TwilightGenConfigBuilder,
    model::{TwilightModel, TwilightModelResult},
    prompt_builder::{TwilightPrompt, TwilightPromptBuilder},
};

#[derive(Debug)]
pub struct TwilightSession<'a> {
    pub(crate) model: &'a TwilightModel,
    pub(crate) config: TwilightGenConfigBuilder,
    pub(crate) prompt: Option<TwilightPromptBuilder>,
}

impl<'a> TwilightSession<'a> {
    pub fn new(model: &'a TwilightModel) -> Self {
        Self {
            model,
            config: TwilightGenConfigBuilder::default(),
            prompt: None,
        }
    }
    pub fn with_config(mut self, config: TwilightGenConfigBuilder) -> Self {
        self.config = config;
        self
    }
    pub fn with_prompt<P>(mut self, prompt: P) -> Self
    where
        P: TwilightPrompt,
    {
        let built = prompt.build().expect("Failed to build prompt");
        self.prompt = Some(built);
        self
    }
    pub fn generate(self) -> anyhow::Result<TwilightModelResult> {
        let prompt = self.prompt.expect("The prompt can't be None!");
        self.model.generate(prompt, self.config)
    }
}
