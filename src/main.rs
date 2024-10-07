mod cli_output;
mod config;
mod steam_api;

use cli_output::show_player_summary;
use config::get_config;
use reqwest::Error;
use steam_api::{get_player_summary, GetPlayerSummaryResponse};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Loggerの初期化
    env_logger::init();

    let config = get_config();

    let response: GetPlayerSummaryResponse = get_player_summary(&config.steam).await?;
    show_player_summary(&response.response.players[0]);

    Ok(())
}
