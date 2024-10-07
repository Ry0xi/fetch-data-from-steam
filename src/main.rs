mod config;
mod steam_api;

use chrono::{Duration, FixedOffset, Utc};
use config::get_config;
use reqwest::Error;
use steam_api::{
    get_player_summary, CommunityVisibilityState, GetPlayerSummaryResponse, PersonState, Player,
};

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
