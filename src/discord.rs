mod bot;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*, utils::token,
};

use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}};

pub struct DiscordBot {
    pub bot: Bot;
}

impl EventHandler for DiscordBot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
            println!("Error sending me: {:?}", why)
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot {} is connected!", ready.user.name);
    }
}
