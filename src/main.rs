use std::env;

use chrono::{DateTime, Duration, FixedOffset, Utc};
use log::debug;
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
    #[serde(with = "chrono::serde::ts_seconds_option")]
    lastlogoff: Option<DateTime<Utc>>,
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
    // Loggerの初期化
    env_logger::init();

    let config = get_config();

    let response: GetPlayerSummaryResponse = get_player_summary(&config.steam).await?;
    show_player_summary(&response.response.players[0]);

    Ok(())
}

fn get_config() -> Config {
    let config = Config {
        steam: SteamConfig {
            api_key: env::var("STEAM_API_KEY").expect("STEAM_API_KEY is not specified."),
            user_id: env::var("STEAM_USER_ID").expect("STEAM_USER_ID is not specified."),
        },
    };

    debug!("API KEY: {}", config.steam.api_key);
    debug!("USER ID: {}", config.steam.user_id);

    config
}

async fn get_player_summary(config: &SteamConfig) -> Result<GetPlayerSummaryResponse, Error> {
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

fn show_player_summary(player: &Player) {
    println!("Name: {}", player.personaname);
    println!("Steam ID: {}", player.steamid);
    println!("Profile: {}", player.profileurl);
    println!("Status: {}", player.personastate);
    println!("Visibility: {}", player.communityvisibilitystate);
    println!(
        "Last logged off at: {}",
        player
            .lastlogoff
            .map(|last_log_off| {
                let now = Utc::now();
                let duration = now - last_log_off;

                // 日本時間に変換
                let jst = FixedOffset::east_opt(9 * 3600).unwrap();
                let last_log_off_jst = last_log_off.with_timezone(&jst);

                format!(
                    "{}前 ( {} )",
                    human_readable_duration(duration),
                    last_log_off_jst.format("%Y年%m月%d日 %H:%M:%S")
                )
            })
            .unwrap_or(String::from("-"))
    );
}

fn human_readable_duration(duration: Duration) -> String {
    let secs = duration.num_seconds();

    const ONE_MINUTE: i64 = 60;
    const ONE_HOUR: i64 = ONE_MINUTE * 60;
    const ONE_DAY: i64 = ONE_HOUR * 24;
    const ONE_YEAR: i64 = ONE_DAY * 365;

    if secs < ONE_MINUTE {
        format!("{}秒", secs)
    } else if secs < ONE_HOUR {
        format!("{}分", secs / 60)
    } else if secs < ONE_DAY {
        format!("{}時間", secs / 3600)
    } else if secs < ONE_DAY * 30 {
        format!("{}日", secs / 86400)
    } else if secs < ONE_YEAR {
        format!("{}ヶ月", secs / (86400 * 30))
    } else {
        format!("{}年", secs / (86400 * 365))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable_duration() {
        let duration = Duration::new(1, 0).unwrap();
        let result = human_readable_duration(duration);
        assert_eq!(result, "1秒");
    }
}
