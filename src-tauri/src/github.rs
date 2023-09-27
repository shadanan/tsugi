use crate::error::TsugiError;
use crate::plugin::{Plugin, Task};
use async_trait::async_trait;

const USER_AGENT: &str = "tsugi - https://github.com/shadanan/tsugi";

#[derive(Clone)]
pub struct AuthenticatedGithubClient {
    client: reqwest::Client,
    token: String,
    user: String,
}

impl AuthenticatedGithubClient {
    pub async fn new() -> Self {
        let client = reqwest::Client::new();

        let token = std::env::var("GITHUB_TOKEN").unwrap();

        let resp = client
            .get("https://api.github.com/user")
            .header("User-Agent", USER_AGENT)
            .bearer_auth(&token)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&resp).unwrap();
        let user = json["login"].as_str().unwrap().to_string();

        Self {
            client,
            token,
            user,
        }
    }

    pub async fn get_search_issues(&self, query: String) -> Result<Vec<Task>, TsugiError> {
        let resp = self
            .client
            .get(format!("https://api.github.com/search/issues?q={query}"))
            .header("User-Agent", USER_AGENT)
            .bearer_auth(&self.token)
            .send()
            .await?
            .text()
            .await?;

        let mut tasks = Vec::new();

        let json: serde_json::Value = serde_json::from_str(&resp)?;

        for item in json["items"].as_array().unwrap_or(&Vec::new()) {
            let task_id = match item["number"].as_i64() {
                Some(number) => number.to_string(),
                None => {
                    eprintln!("Error: Failed to get task number");
                    continue;
                }
            };
            let task = Task {
                key: task_id,
                url: item["html_url"].as_str().unwrap_or("none").to_string(),
                title: item["title"].as_str().unwrap_or("none").to_string(),
                description: "description".to_string(),
                state: item["state"].as_str().unwrap_or("none").to_string(),
                created_at: item["created_at"].as_str().unwrap_or("none").to_string(),
                updated_at: item["updated_at"].as_str().unwrap_or("none").to_string(),
                closed_at: item["closed_at"].as_str().unwrap_or("none").to_string(),
                requestor: item["user"]["login"].as_str().unwrap_or("none").to_string(),
            };
            tasks.push(task);
        }

        Ok(tasks)
    }
}

pub struct GitHubPrReviewPlugin {
    client: AuthenticatedGithubClient,
}

impl GitHubPrReviewPlugin {
    pub fn new(client: &AuthenticatedGithubClient) -> Box<dyn Plugin> {
        Box::new(Self {
            client: client.clone(),
        })
    }
}

#[async_trait]
impl Plugin for GitHubPrReviewPlugin {
    fn name(&self) -> String {
        "GitHub PR Review".to_string()
    }

    async fn tasks(&self) -> Result<Vec<Task>, TsugiError> {
        self.client
            .get_search_issues(format!(
                "is:pr+is:open+archived:false+review-requested:{user}",
                user = self.client.user
            ))
            .await
    }
}

pub struct GitHubPrAuthorPlugin {
    client: AuthenticatedGithubClient,
}

impl GitHubPrAuthorPlugin {
    pub fn new(client: &AuthenticatedGithubClient) -> Box<dyn Plugin> {
        Box::new(Self {
            client: client.clone(),
        })
    }
}

#[async_trait]
impl Plugin for GitHubPrAuthorPlugin {
    fn name(&self) -> String {
        "GitHub PR Author".to_string()
    }

    async fn tasks(&self) -> Result<Vec<Task>, TsugiError> {
        self.client
            .get_search_issues(format!(
                "is:pr+is:open+archived:false+author:{user}",
                user = self.client.user
            ))
            .await
    }
}
