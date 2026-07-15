use std::collections::HashMap;
use std::sync::Arc;
use serenity::all::{Message};
use crate::managers::database_manager::DatabaseManager;
use regex::Regex;
use rusqlite::ToSql;


pub(crate) struct EmojiManager{
    database: Arc<DatabaseManager>,
    regex: Regex
}

impl EmojiManager {
    pub(crate) fn new(database: Arc<DatabaseManager>) -> EmojiManager {
        let manager = EmojiManager { database , regex: Regex::new(r"<a?:[A-Za-z0-9_]{2,32}:\d{17,20}>").unwrap()};
        manager.create_table();

        manager
    }

    fn create_table(&self) {
        let conn = self.database.get_connection();

        conn.execute("CREATE TABLE IF NOT EXISTS emojis (\
        emoji_code VARCHAR(1024) PRIMARY KEY NOT NULL, \
        times_used INTEGER NOT NULL DEFAULT 0 \
        )", ()).expect("Could not create table");

        conn.execute("CREATE TABLE IF NOT EXISTS user_emoji_usage (\
        user_id INTEGER NOT NULL,
        emoji_code VARCHAR(1024) NOT NULL, \
        times_used INTEGER NOT NULL DEFAULT 0, \
        message_has_been_send BOOLEAN NOT NULL DEFAULT FALSE, \
        PRIMARY KEY (user_id, emoji_code)
        )", ()).expect("Could not create table");
    }

    pub(crate) fn contains_emoji(&self, msg: &Message) -> bool{
        self.regex
            .is_match(msg.content.as_str())
    }

    pub(crate) fn save_used_emojis(&self, msg: &Message){

        let mut emoji_map: HashMap<&str, u8> = HashMap::new();

        for val in self.regex.find_iter(msg.content.as_str())
        {
            let counter = emoji_map.entry(val.as_str()).or_insert(0);
            *counter += 1;
        }

        // This might be the most stupid piece of code I have written in my 5 years of making software.
        // iterating over a hashmap with index, dynamically building a query and bindparams for that query.
        let (emoji_sql, user_sql, bind_params) = self.build_full_query(
            msg.author.id.get(),
            &emoji_map
        );

        let params_for = |sql: &str| -> Vec<(&str, &dyn ToSql)> {
            bind_params.iter()
                .filter(|(k, _)| sql.contains(k.as_str()))
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect()
        };

        let binding = self.database.get_connection();
        let transaction = binding.unchecked_transaction().unwrap();

        let first_res = transaction.execute(&emoji_sql, params_for(&emoji_sql).as_slice());
        match first_res {
            Ok(_) =>{}
            Err(e) => {
                println!("Could not execute query {e}");
            }
        }


        let second_res = transaction.execute(&user_sql, params_for(&user_sql).as_slice());
        match second_res {
            Ok(_) =>{}
            Err(e) => {
                println!("Could not execute query {e}");
            }
        }

        let res = transaction.commit();
        match res {
            Ok(_) =>{}
            Err(e) => {
                println!("Could not execute query {e}");
            }
        }

    }


    fn build_full_query<'a>(&self, user_id: u64, emoji_map: &'a HashMap<&str, u8>) -> (String, String, HashMap<String, Box<dyn ToSql + 'a>>){
        let mut emoji_placeholders: String = String::new();
        let mut user_placeholders: String = String::new();
        let mut bind_params: HashMap<String, Box<dyn ToSql>> = HashMap::new();

        let code_param = ":user_id".to_owned();
        bind_params.insert(code_param, Box::new(user_id as i64));


        for (index, (emoji_code, count)) in emoji_map.iter().enumerate(){
            let code_param = format!(":code_{}", index);
            let count_param = format!(":count_{}", index);



            emoji_placeholders.push_str(
                &format!("({},{}),", code_param, count_param)
            );

            user_placeholders.push_str(
                &format!("(:user_id,{},{}),", code_param, count_param)
            );

            bind_params.insert(code_param, Box::new(emoji_code));
            bind_params.insert(count_param, Box::new(*count));
        }
        emoji_placeholders.pop();
        user_placeholders.pop();

        let emoji_sql = format!(
            "INSERT INTO emojis (emoji_code, times_used) VALUES {}
         ON CONFLICT(emoji_code) DO UPDATE SET times_used = times_used + excluded.times_used",
            emoji_placeholders
        );
        let user_sql = format!(
            "INSERT INTO user_emoji_usage (user_id, emoji_code, times_used) VALUES {}
         ON CONFLICT(user_id, emoji_code) DO UPDATE SET times_used = times_used + excluded.times_used",
            user_placeholders
        );

        (emoji_sql, user_sql, bind_params)
    }

}