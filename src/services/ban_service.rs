use std::{thread, time};
use std::sync::Arc;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, Message};
use crate::managers::database_manager::DatabaseManager;

pub(crate) struct BanService{
    ticket_channel_id: u64,
    database: Arc<DatabaseManager>
}

impl BanService {
    pub(crate) fn new(database: Arc<DatabaseManager>, ticket_channel_id: u64) -> BanService{
        Self{ticket_channel_id, database}
    }
    pub(crate) async fn handle_ban(&self, ctx: Context, msg: Message){
        if let Some(guild_id) = msg.guild_id
        {
            msg.delete(&ctx.http).await.expect("Couldnt delete the evil message");
            if let Err(_why) = guild_id.ban(&ctx.http, msg.author.id, 1).await
            {
                let error_message = msg.channel_id.say(&ctx.http, "Couldnt ban this stupid ahh user :pensive:\nSeriously, dont talk in here\n-# this message will be deleted in 3 seconds").await;

                match error_message {
                    Err(e) => println!("Error in sending message {}", e),
                    Ok(new_message) =>{
                        let three_seconds = time::Duration::from_secs(3);
                        thread::sleep(three_seconds);
                        new_message.delete(&ctx.http).await.expect("Couldnt even delete my own message bro");
                    }
                }
            } else {
                let embed = CreateEmbed::new()
                    .title("You have been automatically banned")
                    .description("If your account was taken over, and you want to rejoin Nekotopia, simply just reply with something showing you are not a bot.");
                let builder = CreateMessage::new().embed(embed);
                if let Err(why) = msg.author.dm(&ctx.http, builder).await
                {
                    println!("Could not send dm: {why:?}");
                }
                self.database.add_banned_user(msg.author.id.get());
            }
        }
    }

    pub(crate) async fn handle_appeal(&self, ctx: Context, msg: Message){
        match self.database.get_banned_user(msg.author.id.get())
        {
            Ok(Some(mut user)) => {
                let mut embed = CreateEmbed::new();
                if user.has_appealed{
                    const MAX_STRIKES: u8 = 3;
                    if user.strikes < MAX_STRIKES{
                        user.strikes += 1;
                        embed = embed
                            .title("Avoid spam")
                            .description("To avoid bot/user spam of our ticket system, a strike system has been created.\
                            \nWe expect maybe 1 or 2 real members to want to return for every x bans.\
                            \nYou have a max of 3 strikes, before the bot will no longer send unban requests.");
                        self.create_appeal_message(&ctx, &msg).await;
                        self.database.update_user_stats(user);
                    } else {
                        embed = embed
                            .title("Avoid Spam")
                            .description("Your request has not been send.");
                    }
                } else {
                    user.has_appealed = true;
                    embed = embed
                        .title("Appeal created!")
                        .description("The appeal has been received, when (or if) we realise the tickets channel still exists we will probably unban you.\
                        \nI have not added a re-create invite link yet so ill just add a reinvite link: https://discord.gg/nekotopia\
                        \n***THIS DOES NOT MEAN YOU ARE UNBANNED, IT SIMPLY MEANS THE APPEAL HAS BEEN CREATED***\
                        \n-# it also means that I am too lazy to write an automatic re-invite system for once you have been unbanned.");
                    self.create_appeal_message(&ctx, &msg).await;
                    self.database.update_user_stats(user);
                }
                let builder = CreateMessage::new().embed(embed);
                if let Err(why) = msg.author.dm(&ctx.http, builder).await
                {
                    println!("Could not send dm: {why:?}");
                }
            }
            Ok(None) =>{
                let builder = CreateMessage::new()
                    .content("You are not banned, or you have not been banned by the bot.");
                if let Err(why) = msg.author.dm(&ctx.http, builder).await
                {
                    println!("Could not send dm: {why:?}");
                }
            }
            Err(why)=>{
                println!("Database error: {why:?}")
            }
        }



    }

    pub(crate) async fn create_appeal_message(&self, ctx: &Context, msg: &Message){
        let channel_id = ChannelId::new(self.ticket_channel_id);
        let embed = CreateEmbed::new()
            .title(format!("Unban request created by: {}({})", msg.author.name, msg.author.id.get()))
            .description(format!("content:\n{}", msg.content));
        let builder = CreateMessage::new().embed(embed);

        if let Err(why) = channel_id.send_message(&ctx.http,builder).await
        {
            println!("Could not send dm: {why:?}");
        }

    }
}