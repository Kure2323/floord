use rusqlite::Connection;
use std::fs::create_dir_all;

#[derive(Debug)]
struct Floor {
    file_name: String,
    last_date: String,
    size_in_bytes: u64
}

pub fn init() -> bool {
    let db_path = dirs::data_dir().unwrap().join("floord").join("floord.db");
    let _ = create_dir_all(db_path.parent().unwrap());
    if let Ok(con) = Connection::open(&db_path) {
        con.execute(
            "CREATE TABLE IF NOT EXISTS floord (
            file_name TEXT NOT NULL,
            last_date TEXT NOT NULL,
            size_in_bytes INT
            )",
            ()
        ).expect("[ERROR] Could not create database table");
        return true;
    }
    false
}