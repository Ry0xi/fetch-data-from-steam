use std::env;

use reqwest::Error;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

struct Config {
    steam: SteamConfig,
}

struct SteamConfig {
    api_key: String,
    user_id: String,
}

#[derive(Deserialize)]
struct GetPlayerSummaryResponse {
    response: GetPlayerSummaryResponseInner,
}

#[derive(Deserialize)]
struct GetPlayerSummaryResponseInner {
    players: Vec<Player>,
}

#[derive(Deserialize)]
struct Player {
    steamid: String,
    personaname: String,
    profileurl: String,
    personastate: PersonState,
    communityvisibilitystate: CommunityVisibilityState,
    lastlogoff: Option<u32>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
enum PersonState {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for PersonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            PersonState::Offline => "オフライン",
            PersonState::Online => "オンライン",
            PersonState::Busy => "取り込み中",
            PersonState::Away => "退席中",
            PersonState::Snooze => "スヌーズ",
            PersonState::LookingToTrade => "トレード希望",
            PersonState::LookingToPlay => "プレイ希望",
            PersonState::Unknown => "不明",
        };
        write!(f, "{}", state_str)
    }
}

#[derive(Deserialize_repr)]
#[repr(u8)]
enum CommunityVisibilityState {
    NotVisible = 1,
    Visible = 3,
}

impl std::fmt::Display for CommunityVisibilityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            CommunityVisibilityState::NotVisible => "閲覧不可能",
            CommunityVisibilityState::Visible => "閲覧可能",
        };
        write!(f, "{}", state_str)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = get_config();
    println!("API KEY: {}", config.steam.api_key);
    println!("USER ID: {}", config.steam.user_id);

    let response: GetPlayerSummaryResponse = get_player_summary(&config.steam).await?;
    show_player_summary(&response.response.players[0]);

    Ok(())
}

fn get_config() -> Config {
    Config {
        steam: SteamConfig {
            api_key: env::var("STEAM_API_KEY").expect("STEAM_API_KEY is not specified."),
            user_id: env::var("STEAM_USER_ID").expect("STEAM_USER_ID is not specified."),
        }
    }
}

async fn get_player_summary(config: &SteamConfig) -> Result<GetPlayerSummaryResponse, Error> {
    let url = format!(
        "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
        config.api_key,
        config.user_id,
    );

    println!("URL: {}", url);

    reqwest::get(&url).await?.json::<GetPlayerSummaryResponse>().await
}

fn show_player_summary(player: &Player) {
    println!("Name: {}", player.personaname);
    println!("Steam ID: {}", player.steamid);
    println!("Profile: {}", player.profileurl);
    println!("Status: {}", player.personastate);
    println!("Visibility: {}", player.communityvisibilitystate);
    println!("Last log off: {}", player.lastlogoff.map(|timestamp| format!("{}", timestamp)).unwrap_or(String::from("-")));
}
