
use std::sync::Arc;
use rusqlite::named_params;
use crate::managers::database_manager::DatabaseManager;

pub(crate) struct BanManager{
    database: Arc<DatabaseManager>
}

// pub (crate) struct BannedUser{
//     pub(crate) user_id: u64,
//     pub(crate) banned_times: u8
// }


impl BanManager{
    pub(crate) fn new(database: Arc<DatabaseManager>) -> BanManager{
        let manager = BanManager{database};
        manager.create_table();

        manager
    }

    fn create_table(&self){
        let conn = self.database.get_connection();

        conn.execute("CREATE TABLE IF NOT EXISTS banned_users (\
        user_id INTEGER PRIMARY KEY, \
        banned_times BOOLEAN NOT NULL \
        )", ()).expect("Could not create table");
    }

    pub(crate)fn add_banned_user(&self, user_id: u64){

        let conn = self.database.get_connection();

        let mut stmt = conn.prepare(
            "INSERT INTO banned_users(user_id, banned_times) \
        VALUES (:id, 0)\
        ON CONFLICT(user_id) DO UPDATE SET banned_times = banned_times + 1"
        ).unwrap();
        stmt.execute(
            named_params! {
                ":id": user_id as i64
            }
        ).expect("Could not add user");
    }

    // pub(crate) fn get_banned_user(&self, user_id: u64) -> rusqlite::Result<Option<BannedUser>>{
    //     let conn = self.database.get_connection();
    //     let mut stmt = conn.prepare(
    //         "SELECT user_id, banned_times FROM banned_users WHERE user_id = :id")?;
    //
    //     stmt.query_one(named_params! {":id": user_id as i64}, |row| {
    //         Ok(BannedUser{
    //             user_id: row.get::<_, i64>(0)? as u64,
    //             banned_times: row.get(1)?,
    //         })
    //     }).optional()
    // }
    //
    // pub(crate) fn update_user_stats(&self, user: BannedUser){
    //     let conn = self.database.get_connection();
    //
    //     let mut stmt = conn.prepare(
    //         "UPDATE banned_users SET has_appealed = :banned_times WHERE user_id = :id"
    //     ).unwrap();
    //     stmt.execute(
    //         named_params! {
    //             ":id": user.user_id as i64,
    //             ":banned_times": user.banned_times
    //         }
    //     ).expect("Could not add user");
    // }
}
