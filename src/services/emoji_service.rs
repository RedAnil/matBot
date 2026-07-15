use crate::managers::emoji_manager::EmojiManager;
use serenity::all::Message;

pub(crate) struct EmojiService{
    manager: EmojiManager
}

impl EmojiService{
    pub(crate) fn new(manager: EmojiManager) -> EmojiService
    {
        Self{manager}
    }

    pub(crate) fn contains_emoji(&self, msg: &Message) -> bool{
        self.manager.contains_emoji(msg)
    }

    pub(crate) fn save_emoji_usages(&self, msg: &Message){
        self.manager.save_used_emojis(msg);
    }
}