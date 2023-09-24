use crate::error::TsugiError;
use crate::plugin::Plugin;
use crate::task::Task;
use async_trait::async_trait;

const USER_AGENT: &str = "tsugi - https://github.com/shadanan/tsugi";

pub struct AuthenticatedGithubClient {
    client: reqwest::Client,
    token: String,
    user: String,
}

pub async fn init() -> Box<dyn Plugin> {
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

    let client = AuthenticatedGithubClient {
        client,
        token,
        user,
    };

    Box::new(client)
}

impl AuthenticatedGithubClient {
    async fn get_search_issues(
        &self,
        query: String,
        kind: String,
    ) -> Result<Vec<Task>, TsugiError> {
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
                kind: kind.clone(),
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

#[async_trait]
impl Plugin for AuthenticatedGithubClient {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    async fn tasks(&self) -> Result<Vec<Task>, TsugiError> {
        let reviews = self
            .get_search_issues(
                format!(
                    "is:pr+is:open+archived:false+review-requested:{user}",
                    user = self.user
                ),
                "GitHub PR Review".to_string(),
            )
            .await?;

        let myprs = self
            .get_search_issues(
                format!(
                    "is:pr+is:open+archived:false+author:{user}",
                    user = self.user
                ),
                "GitHub PR".to_string(),
            )
            .await?;

        let mut tasks = Vec::new();
        tasks.extend(reviews);
        tasks.extend(myprs);

        Ok(tasks)
    }
}
