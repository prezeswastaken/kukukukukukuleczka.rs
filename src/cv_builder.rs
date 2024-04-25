use crate::forms::BasicForm;
use anyhow::anyhow;
use openai_api_rust::chat::*;
use openai_api_rust::completions::*;
use openai_api_rust::*;

pub struct CVBuilder {
    prompt: String,
}

impl From<BasicForm> for CVBuilder {
    fn from(form: BasicForm) -> Self {
        let prompt = Prompt::new()
            .add_section("Pełne imię i nazwisko", form.full_name)
            .add_section("Email", form.email)
            .add_list_section("Znane języki programowania", form.programming_languages)
            .add_section("Poziom edukacji", form.education_level)
            .prompt;

        Self { prompt }
    }
}

impl CVBuilder {
    pub fn create_cv_string(&self) -> anyhow::Result<String> {
        let auth = Auth::from_env().map_err(|_| anyhow!("Couldn't read api key from env file"))?;
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: None,
            temperature: Some(0_f32),
            top_p: Some(0_f32),
            n: Some(2),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![Message {
                role: Role::User,
                content: self.prompt.clone(),
            }],
        };
        let response = openai.chat_completion_create(&body);
        let choice = response
            .map_err(|_| anyhow!("Something went wrong with response :("))?
            .choices;
        let message = &choice[0]
            .message
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't get message from response"))?;
        Ok(message.content.clone())
    }
}

struct Prompt {
    prompt: String,
}

impl Prompt {
    fn new() -> Self {
        Self {
            prompt: "Napisz dla mnie CV, którego mogę użyć do znalezienia pracy, bazując na podanych danych.".to_string(),
        }
    }

    fn add_section(&self, name: &str, value: String) -> Self {
        if value.is_empty() {
            return Self {
                prompt: self.prompt.clone(),
            };
        }
        let new_prompt = format!("{}\n{}: {}", self.prompt, name, value);
        Self { prompt: new_prompt }
    }

    fn add_list_section(&self, name: &str, values: Vec<String>) -> Self {
        if values.is_empty() {
            return Self {
                prompt: self.prompt.clone(),
            };
        }
        let new_prompt = format!("{}\n{}: {}", self.prompt, name, values.join(", "));
        Self { prompt: new_prompt }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        let prompt = Prompt::new()
            .add_section("Full Name", "John Doe".to_string())
            .add_section("Email", "test@test.com".to_string())
            .add_list_section(
                "Programming languages",
                vec!["Python".to_string(), "Rust".to_string()],
            );
        assert_eq!(prompt.prompt, "Napisz dla mnie CV, którego mogę użyć do znalezienia pracy, bazując na podanych danych.\nFull Name: John Doe\nEmail: test@test.com\nProgramming languages: Python, Rust")
    }
}
