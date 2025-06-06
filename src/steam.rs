use reqwest::Url;

use crate::env;

/// if a url is a vanity url
/// vanity url is a url that looks like this: https://steamcommunity.com/id/username
pub fn is_vanity_url(url: &str) -> bool {
    let parsed_url = Url::parse(url).unwrap();
    let segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();
    segments.len() >= 2 && segments[0] == "id"
}

pub async fn get_steam_id_from_vanity_url(url: &str) -> Option<String> {
    let username = get_username_from_vanity_url(url);

    if let Some(username) = username {
        let api_url = format!(
            "https://api.steampowered.com/ISteamUser/ResolveVanityURL/v1/?key={}&vanityurl={}",
            env::get("STEAM_API_KEY"),
            username
        );
        let response = reqwest::get(&api_url).await.ok()?;

        let json: serde_json::Value = response.json().await.ok()?;
        if let Some(steam_id) = json["response"]["steamid"].as_str() {
            return Some(steam_id.to_string());
        }
    }

    None
}

pub fn get_steam_id_from_non_vanity_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).unwrap();
    let segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();

    if segments.len() < 2 || segments[0] != "profiles" {
        None
    } else {
        Some(segments[1].to_string())
    }
}

fn get_username_from_vanity_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).unwrap();
    let segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();

    if segments.len() < 2 || segments[0] != "id" {
        None
    } else {
        Some(segments[1].to_string())
    }
}

pub fn normalize_url(url: &str) -> Result<String, String> {
    let mut parsed_url = Url::parse(url).unwrap();
    parsed_url.set_query(None);
    parsed_url.set_fragment(None);

    let segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();
    if segments.len() >= 2 && (segments[0] == "id" || segments[0] == "profiles") {
        let base_path = format!("{}/{}", segments[0], segments[1]);
        parsed_url.set_path(&base_path);
        return Ok(parsed_url.to_string());
    }

    Err("Invalid URL format".to_string())
}
