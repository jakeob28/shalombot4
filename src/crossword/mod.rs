mod puzzle_time_utils;

use crate::crossword::puzzle_time_utils::puzzle_period;
use chrono::NaiveDate;
use log::{debug, error, info, warn};

use serde_json::Value;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

use serenity::utils::Colour;

const CROSSWORD_CHANNEL: u64 = 765753596532359190;

pub async fn start_crossword_watch(ctx: Context) {
    info!("Starting crossword watch...");
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut cw: CrosswordWatcher = CrosswordWatcher {
            ctx,
            last_posted_puzzle: None,
        };
        loop {
            interval.tick().await;
            cw.check_crossword().await;
        }
    });
}

struct CrosswordWatcher {
    ctx: Context,
    last_posted_puzzle: Option<NaiveDate>,
}

impl CrosswordWatcher {
    async fn check_crossword(&mut self) {
        let date = query_nyt_latest_puzzle_date().await;
        if date.is_err() {
            warn!(
                "Failed to get latest crossword from NYT: {}",
                date.unwrap_err()
            );
            return;
        }
        if !&self.is_already_sent(date.as_ref().unwrap()).await {
            self.send_crossword_message(date.unwrap()).await;
        }
    }

    async fn send_crossword_message(&self, date: NaiveDate) {
        let period = puzzle_period(&date);
        match ChannelId(CROSSWORD_CHANNEL).send_message(&self.ctx, |m| {
            m.embed(|e| {
                e.title(date.format("%A, %B %-d, %Y"))
                    .description("https://www.nytimes.com/crosswords/game/mini")
                    .thumbnail("https://cdn.discordapp.com/attachments/694653665910456322/1071919797819936798/mini-progress-0.png")
                    .color(Colour::from(0x00ffff))
                    .field("Start", format!("<t:{}:R>", period.0.timestamp()), true)
                    .field("End", format!("<t:{}:R>", period.1.timestamp()), true)
            }).components(|f| {
                f.create_action_row(|r| {
                    r.create_button(|btn| {
                        btn.style(ButtonStyle::Link).url("https://www.nytimes.com/crosswords/game/mini").label("Play")
                    })
                })
            })
        }).await {
            Ok(_) => { info!("Crossword message sent!") }
            Err(why) => { error!("Error sending crossword message: {}", why) }
        }
    }

    async fn is_already_sent(&mut self, date: &NaiveDate) -> bool {
        if let Some(last_posted_puzzle) = &self.last_posted_puzzle {
            if last_posted_puzzle == date {
                debug!("No message scan necessary");
                return true;
            }
        }
        debug!("Scanning messages for last posted puzzle");
        let mut messages = ChannelId(CROSSWORD_CHANNEL)
            .messages_iter(&self.ctx)
            .boxed();
        while let Some(message_result) = messages.next().await {
            match message_result {
                Ok(message) => {
                    let puzzle_date_of_msg =
                        puzzle_time_utils::puzzle_date_from_timestamp(message.timestamp);
                    if puzzle_date_of_msg < *date {
                        debug!("No posted puzzle found today");
                        return false;
                    }
                    if message_is_crossword_post(message) {
                        debug!("Found puzzle message already posted");
                        self.last_posted_puzzle = Some(puzzle_date_of_msg);
                        return true;
                    }
                }
                Err(error) => error!("Error retrieving channel messages: {}", error),
            }
        }
        false
    }
}

async fn query_nyt_latest_puzzle_date() -> Result<NaiveDate, reqwest::Error> {
    let body = reqwest::get("https://www.nytimes.com/svc/crosswords/v6/puzzle/mini.json")
        .await?
        .text()
        .await?;
    let json: Value = serde_json::from_str(&body).expect("Error parsing JSON");
    let date = NaiveDate::parse_from_str(json["publicationDate"].as_str().unwrap(), "%Y-%m-%d")
        .expect("Bad date format");
    Ok(date)
}

fn message_is_crossword_post(message: Message) -> bool {
    if message
        .content
        .contains("https://www.nytimes.com/crosswords/game/mini")
    {
        return true;
    }
    for embed in message.embeds {
        if let Some(desc) = embed.description {
            if desc.contains("https://www.nytimes.com/crosswords/game/mini") {
                return true;
            }
        }
    }
    false
}
