use rusqlite::Connection;
use std::path::Path;
struct Db {
    connection: Connection,
}

impl Db {
    fn new(db_path: &Path) -> Db {
        let connection = Connection::open(db_path).unwrap();
        Db { connection }
    }
}
