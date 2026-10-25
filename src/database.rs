use rusqlite::Connection;

#[derive(Debug)]
struct File {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn init() {
    let conn = Connection::open("");
}