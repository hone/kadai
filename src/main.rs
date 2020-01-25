use serde_derive::Deserialize;
use reqwest::header;

const USER_AGENT: &str = "Kodai";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", header::HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder().user_agent(USER_AGENT).default_headers(headers).build()?;

    let url = format!("https://api.github.com/repos/{owner}/{repo}/issues",
       owner = "rust-lang-nursery",
       repo = "rust-cookbook");
    println!("GET {}", &url);
    let req = client.get(&url).query(&("state", "open"));
    let resp = req.send().await?;
    if resp.status().is_success() {
        println!("{:#?}", resp);
    }

    Ok(())
}
