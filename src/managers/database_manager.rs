use rusqlite::{Connection};
use std::sync::{Mutex, MutexGuard};

pub(crate) struct DatabaseManager{
    conn: Mutex<Connection>,

}


impl DatabaseManager {

    pub(crate) fn new(db_name: &str) -> Self{
        let conn = Connection::open(db_name).unwrap();
        Self {conn: Mutex::new(conn)}

    }

    pub(crate) fn get_connection(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }


}