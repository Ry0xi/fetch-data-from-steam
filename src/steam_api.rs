use chrono::{DateTime, Utc};
use log::debug;
use reqwest::Error;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::config::SteamConfig;

#[derive(Deserialize)]
pub struct GetPlayerSummaryResponse {
    pub response: GetPlayerSummaryResponseInner,
}

#[derive(Deserialize)]
pub struct GetPlayerSummaryResponseInner {
    pub players: Vec<Player>,
}

#[derive(Deserialize)]
pub struct Player {
    pub steamid: String,
    pub personaname: String,
    pub profileurl: String,
    pub personastate: PersonState,
    pub communityvisibilitystate: CommunityVisibilityState,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub lastlogoff: Option<DateTime<Utc>>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum PersonState {
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

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum CommunityVisibilityState {
    NotVisible = 1,
    Visible = 3,
}

pub async fn get_player_summary(config: &SteamConfig) -> Result<GetPlayerSummaryResponse, Error> {
    let url = format!(
        "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
        config.api_key, config.user_id,
    );

    debug!("GET Player Summary URL: {}", url);

    reqwest::get(&url)
        .await?
        .json::<GetPlayerSummaryResponse>()
        .await
}
