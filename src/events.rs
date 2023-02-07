use log::info;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, Ordering};

use serenity::model::gateway::Ready;

use crate::crossword;

pub struct Handler {
    pub(crate) is_watch_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if !self.is_watch_running.load(Ordering::Relaxed) {
            info!("{} is connected!", ready.user.name);
            crossword::start_crossword_watch(Context::clone(&ctx)).await;
            self.is_watch_running.store(true, Relaxed)
        }
    }
}
