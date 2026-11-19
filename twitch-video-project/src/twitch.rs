use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub duration: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}
#[derive(Deserialize)]
struct UserResponse {
    data: Vec<User>,
}
#[derive(Deserialize)]
struct User {
    id: String,
}
#[derive(Deserialize)]
struct VideoResponse {
    data: Vec<ApiVideo>,
    pagination: Pagination,
}
#[derive(Deserialize)]
struct ApiVideo {
    id: String,
    title: String,
    created_at: String,
    duration: String,
}
#[derive(Deserialize, Default)]
struct Pagination {
    cursor: Option<String>,
}

pub fn list_videos(
    login: &str,
    client_id: Option<&str>,
    client_secret: Option<&str>,
) -> Result<Vec<Video>, String> {
    let client_id = client_id
        .map(str::to_owned)
        .or_else(|| std::env::var("TWITCH_CLIENT_ID").ok())
        .filter(|s| !s.trim().is_empty())
        .ok_or("missing Twitch client ID; pass --client-id or set TWITCH_CLIENT_ID")?;
    let client_secret = client_secret
        .map(str::to_owned)
        .or_else(|| std::env::var("TWITCH_CLIENT_SECRET").ok())
        .filter(|s| !s.trim().is_empty())
        .ok_or("missing Twitch client secret; pass --client-secret or set TWITCH_CLIENT_SECRET")?;
    let client = Client::builder()
        .user_agent("tvd/0.1")
        .build()
        .map_err(|e| format!("could not create HTTP client: {e}"))?;

    let token: TokenResponse = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| format!("Twitch authentication failed: {e}"))?
        .json()
        .map_err(|e| format!("invalid Twitch token response: {e}"))?;

    let response: UserResponse = helix(
        &client,
        &client_id,
        &token.access_token,
        "https://api.twitch.tv/helix/users",
        &[("login", login)],
    )?;
    let user_id = response
        .data
        .first()
        .ok_or_else(|| format!("Twitch channel '{login}' not found"))?
        .id
        .clone();

    let mut videos = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut query = vec![
            ("user_id".to_owned(), user_id.clone()),
            ("first".to_owned(), "100".to_owned()),
        ];
        if let Some(value) = cursor.take() {
            query.push(("after".to_owned(), value));
        }
        let query_ref: Vec<(&str, &str)> = query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let page: VideoResponse = helix(
            &client,
            &client_id,
            &token.access_token,
            "https://api.twitch.tv/helix/videos",
            &query_ref,
        )?;
        videos.extend(page.data.into_iter().map(|v| Video {
            id: v.id,
            title: v.title,
            created_at: v.created_at,
            duration: v.duration,
        }));
        cursor = page.pagination.cursor.filter(|v| !v.is_empty());
        if cursor.is_none() {
            break;
        }
    }
    Ok(videos)
}

fn helix<T: for<'de> Deserialize<'de>>(
    client: &Client,
    id: &str,
    token: &str,
    url: &str,
    query: &[(&str, &str)],
) -> Result<T, String> {
    client
        .get(url)
        .query(query)
        .header("Client-Id", id)
        .bearer_auth(token)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| format!("Twitch API request failed: {e}"))?
        .json()
        .map_err(|e| format!("invalid Twitch API response: {e}"))
}
