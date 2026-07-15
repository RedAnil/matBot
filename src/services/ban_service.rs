use serenity::all::{Context, CreateEmbed, CreateMessage, Message};
use std::{thread, time};
use crate::managers::ban_manager::BanManager;

pub(crate) struct BanService{
    manager: BanManager
}

impl BanService {
    pub(crate) fn new(manager: BanManager) -> BanService{
        Self{manager}
    }
    pub(crate) async fn handle_ban(&self, ctx: Context, msg: Message){
        if let Some(guild_id) = msg.guild_id
        {
            msg.delete(&ctx.http).await.expect("Couldnt delete the evil message");
            let embed = CreateEmbed::new()
                .title("You have been automatically banned.")
                .description("The bot has automatically banned you for talking in the no talk channel\
                        \nIf you get your account back (or were just too curious) and want to rejoin back: https://discord.gg/nekotopia\
                        \n***Give it a second and you should be unbanned, bot is slow.***\
                        \n-# it has to also mark you as banned, it keeps track of how many times it has banned you.");
            let builder = CreateMessage::new().embed(embed);
            if let Err(why) = msg.author.dm(&ctx.http, builder).await
            {
                println!("Could not send dm: {why:?}");
            }
            if let Err(_why) = guild_id.ban(&ctx.http, msg.author.id, 1).await
            {
                let error_message = msg.channel_id.say(&ctx.http, "Couldnt ban this stupid ass user :pensive:\nSeriously, dont talk in here\n-# this message will be deleted in 3 seconds").await;

                match error_message {
                    Err(e) => println!("Error in sending message {}", e),
                    Ok(new_message) =>{
                        let three_seconds = time::Duration::from_secs(3);
                        thread::sleep(three_seconds);
                        new_message.delete(&ctx.http).await.expect("Couldnt even delete my own message bro");
                    }
                }
            } else {
                self.manager.add_banned_user(msg.author.id.get());
                if let Err(_why) = guild_id.unban(&ctx.http, msg.author.id).await{
                  msg.channel_id.say(&ctx.http, format!("Could not unban user {}({})\n-# Check the logs or something.", msg.author.name, msg.author.id.get()))
                      .await
                      .expect("Error in sending unban message");
                }
            }
        }
    }


}