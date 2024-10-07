use log::debug;
use std::env;

pub struct Config {
    pub steam: SteamConfig,
}

pub struct SteamConfig {
    pub api_key: String,
    pub user_id: String,
}

pub fn get_config() -> Config {
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
