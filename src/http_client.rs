use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub url: String,
}

pub struct HttpClient {
    agent: ureq::Agent,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new().build(),
        }
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        let response = self.agent.get(url).call().map_err(|e| e.to_string())?;
        self.parse_response(response, url)
    }

    fn parse_response(&self, response: ureq::Response, url: &str) -> Result<HttpResponse, String> {
        let status = response.status();
        let headers = response
            .headers_names()
            .into_iter()
            .filter_map(|name| {
                response
                    .header(&name)
                    .map(|value| (name, value.to_string()))
            })
            .collect();

        let body = response.into_string().map_err(|e| e.to_string())?;

        Ok(HttpResponse {
            status,
            headers,
            body,
            url: url.to_string(),
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
