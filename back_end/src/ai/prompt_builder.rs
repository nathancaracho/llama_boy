use core::task;

#[derive(Debug, Clone, Default)]
pub struct TwilightPromptBuilder {
    pub(crate) content: String,
}
impl TwilightPromptBuilder {
    pub fn append(mut self, section: &str) -> Self {
        self.content.push_str(section);
        self
    }

    pub fn build(self) -> String {
        self.content
    }
}
pub trait TwilightPrompt {
    fn build(self) -> anyhow::Result<TwilightPromptBuilder>;
}

#[derive(Debug, Default)]
pub struct TwilightRolePrompt {
    pub role: String,
    pub task: String,
}

impl TwilightPrompt for TwilightRolePrompt {
    fn build(self) -> anyhow::Result<TwilightPromptBuilder> {
        let section = format!(
            r#"
# Role:
{role}
## Task:
{task}

"#,
            role = self.role,
            task = self.task
        );

        Ok(TwilightPromptBuilder::default().append(&section))
    }
}
