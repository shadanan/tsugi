use crate::task;
use futures::join;

const USER_AGENT: &str = "tsugi - https://github.com/shadanan/tsugi";

pub struct AuthenticatedGithubClient {
    client: reqwest::Client,
    token: String,
    user: String,
}

pub async fn init() -> AuthenticatedGithubClient {
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

    AuthenticatedGithubClient {
        client,
        token,
        user,
    }
}

impl AuthenticatedGithubClient {
    async fn get_search_issues(&self, query: String, kind: String) -> Vec<task::Task> {
        let resp = self
            .client
            .get(format!("https://api.github.com/search/issues?q={query}"))
            .header("User-Agent", USER_AGENT)
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut tasks = Vec::new();

        let json: serde_json::Value = serde_json::from_str(&resp).unwrap();

        for item in json["items"].as_array().unwrap() {
            let task_id = item["number"].as_i64().unwrap().to_string();
            let task = task::Task {
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

        tasks
    }

    pub async fn get_tasks(&self) -> Vec<task::Task> {
        let reviews = self.get_search_issues(
            format!(
                "is:pr+is:open+archived:false+review-requested:{user}",
                user = self.user
            ),
            "GitHub PR Review".to_string(),
        );

        let myprs = self.get_search_issues(
            format!(
                "is:pr+is:open+archived:false+author:{user}",
                user = self.user
            ),
            "GitHub PR".to_string(),
        );

        let (reviews, myprs) = join!(reviews, myprs);
        let mut tasks = Vec::new();
        tasks.extend(reviews);
        tasks.extend(myprs);

        tasks
    }
}
