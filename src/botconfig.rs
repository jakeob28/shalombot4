use config::{Config, Environment, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GuildSettings {
    pub(crate) guild_id: u64,
    pub(crate) crossword_channel: u64,
}
#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub(crate) discord_token: String,
    pub(crate) guild_settings: GuildSettings,
    pub(crate) embed_color: i32,
}

impl BotConfig {
    pub fn global_cfg() -> &'static Self {
        static INSTANCE: OnceCell<BotConfig> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let s = Config::builder()
                .add_source(File::with_name("config/default.yaml"))
                .add_source(File::with_name("config/local.yaml").required(false))
                .add_source(Environment::with_prefix("shalom"))
                .build()
                .unwrap();
            // You can deserialize (and thus freeze) the entire configuration as
            s.try_deserialize().unwrap()
        })
    }
}
