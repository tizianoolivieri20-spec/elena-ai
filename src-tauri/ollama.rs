use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    response: String,
}

pub struct OllamaClient {
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            model: "mistral".to_string(), // Default model, can be changed
        }
    }

    pub async fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let full_prompt = format!("{}\n\n{}", system_prompt, user_prompt);

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: full_prompt,
            stream: false,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        let data: OllamaResponse = response.json().await?;
        Ok(data.response)
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}
