use std::env;

struct Config {
    steam: SteamConfig,
}

struct SteamConfig {
    api_key: String,
    user_id: String,
}

fn main() {
    let config = get_config();
    println!("API KEY: {}", config.steam.api_key);
    println!("USER ID: {}", config.steam.user_id);
}

fn get_config() -> Config {
    Config {
        steam: SteamConfig {
            api_key: env::var("STEAM_API_KEY").expect("STEAM_API_KEY is not specified."),
            user_id: env::var("STEAM_USER_ID").expect("STEAM_USER_ID is not specified."),
        }
    }
}
