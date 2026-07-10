mod managers;
mod services;

use std::{env};
use std::sync::Arc;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use managers::database_manager::DatabaseManager;
use crate::services::ban_service::BanService;

struct Handler{
    ban_channel_id: u64,
    ban_service: BanService,
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.author.bot {
            return;
        }

        if msg.guild_id.is_none(){
            self.ban_service.handle_appeal(ctx, msg).await;
            return;
        }

        if msg.channel_id.get() == self.ban_channel_id{
            self.ban_service.handle_ban(ctx, msg).await;
            return;

        }
    }


}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILD_MODERATION;

    let ban_channel_id: u64 = env::var("BAN_CHANNEL")
        .expect("Expected a ban channel id")
        .parse()
        .expect("BAN_CHANNEL must be unsigned 64");

    let tickets_channel_id: u64 = env::var("TICKET_CHANNEL")
        .expect("Expected a ticket channel id")
        .parse()
        .expect("TICKET_CHANNEL must be unsigned 64");

    let database = Arc::new(

        DatabaseManager::new("nekotopia.db")
    );

    let handler = Handler {
        ban_channel_id,
        ban_service: BanService::new(Arc::clone(&database), tickets_channel_id)
    };

    let mut client = Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");

    if let Err(why) = client.start().await{
        println!("Client error: {why:?}");
    }
}
