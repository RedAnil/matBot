use rusqlite::{named_params, Connection, OptionalExtension};
use std::sync::Mutex;

pub(crate) struct DatabaseManager{
    conn: Mutex<Connection>,

}


pub (crate) struct BannedUser{
    pub(crate) user_id: u64,
    pub(crate) has_appealed: bool,
    pub(crate) strikes: u8
}

impl DatabaseManager {

    pub(crate) fn new(db_name: &str) -> Self{
        let conn = Connection::open(db_name).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS banned_users (\
        user_id INTEGER PRIMARY KEY, \
        has_appealed BOOLEAN NOT NULL, \
        strikes INTEGER NOT NULL\
        )", ()).expect("Could not create table");



        Self {conn: Mutex::new(conn)}

    }

    pub(crate)fn add_banned_user(&self, user_id: u64){

        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "INSERT INTO banned_users(user_id, has_appealed, strikes) \
        VALUES (:id, false, 0)\
        ON CONFLICT(user_id) DO UPDATE SET strikes = 0, has_appealed = false"
        ).unwrap();
        stmt.execute(
            named_params! {
                ":id": user_id as i64
            }
        ).expect("Could not add user");
    }

    pub(crate) fn get_banned_user(&self, user_id: u64) -> rusqlite::Result<Option<BannedUser>>{
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT user_id, has_appealed, strikes FROM banned_users WHERE user_id = :id")?;

        stmt.query_one(named_params! {":id": user_id as i64}, |row| {
            Ok(BannedUser{
                user_id: row.get::<_, i64>(0)? as u64,
                has_appealed: row.get(1)?,
                strikes: row.get(2)?,
            })
        }).optional()
    }

    pub(crate) fn update_user_stats(&self, user: BannedUser){
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "UPDATE banned_users SET has_appealed = :has_appealed, strikes = :strikes WHERE user_id = :id"
        ).unwrap();
        stmt.execute(
            named_params! {
                ":id": user.user_id as i64,
                ":has_appealed": user.has_appealed,
                ":strikes": user.strikes
            }
        ).expect("Could not add user");
    }
}