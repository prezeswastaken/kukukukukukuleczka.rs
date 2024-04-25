use std::{env::current_dir, fs::File, io::Write, path::Path, process::Command};

use anyhow::anyhow;
use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};

pub struct PdfBuilder {
    cv_string: String,
    pub latex_file_name: Option<String>,
    pdf_file_name: Option<String>,
    storage_path: String,
}

impl PdfBuilder {
    pub fn new(cv_string: String, storage_path: String) -> Self {
        Self {
            cv_string,
            latex_file_name: None,
            pdf_file_name: None,
            storage_path,
        }
    }

    pub fn create_latex_file(&mut self) -> anyhow::Result<()> {
        let uuid_string = uuid::Uuid::new_v4().to_string();
        let file_name = format!("{}.tex", uuid_string);

        let auth = Auth::from_env().map_err(|_| anyhow!("Couldn't read api key from env file"))?;
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        let prompt = format!("This is my text for CV: {}\n. Create a latex file from it. It should use only default packages. At the start, use fontenc and inputenc to properly encode polish characters.", self.cv_string);

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
                content: prompt,
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

        let latex_content = message.content.clone();
        if latex_content.is_empty() {
            return Err(anyhow!("Couldn't get latex content from response"));
        }
        let file_path = Path::new(&self.storage_path).join(&file_name);
        if file_path.parent().is_none() {
            std::fs::create_dir_all(file_path.parent().unwrap())?;
        }
        println!("Creating file: {}", file_path.display());
        File::create(&file_path)?.write_all(latex_content.as_bytes())?;
        println!("File created successfully");
        self.latex_file_name = Some(file_name);

        Ok(())
    }

    pub fn create_pdf(&mut self) -> anyhow::Result<String> {
        let latex_file_name = self
            .latex_file_name
            .as_ref()
            .ok_or_else(|| anyhow!("Latex file name is not set"))?;
        let latex_file_path = Path::new(&self.storage_path).join(latex_file_name);
        let pdf_file_name = latex_file_name.replace(".tex", ".pdf");

        Command::new("pdflatex")
            .arg("-output-directory")
            .arg(&self.storage_path)
            .arg(&latex_file_path)
            .output()
            .map_err(|_| anyhow!("Couldn't create pdf file"))?;

        Ok(pdf_file_name.to_string())
    }
}
