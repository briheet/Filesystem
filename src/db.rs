use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
};
#[derive(Debug)]
pub struct Db {
    path: PathBuf,
    connection: Connection,
}

impl Db {
    pub fn new(path: PathBuf) -> Db {
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        let sqlite_path = path.join("metadata.db");
        let connection = Connection::open(sqlite_path).unwrap();
        Db { path, connection }
    }

    pub fn create_item() {}
}
