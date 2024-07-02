use rusqlite::{Connection, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum CreateItemError {
    ItemExists,
}

#[derive(Debug)]
pub struct Db {
    path: PathBuf,
    connection: Connection,
}

#[derive(Debug)]
pub struct DbItem {
    // Our Db item is gonna have path to the item and his name
    path: PathBuf,
    pub name: String,
}

impl Db {
    pub fn new(path: PathBuf) -> Db {
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        let sqlite_path = path.join("metadata.db");
        let connection = Connection::open(sqlite_path).unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS files(id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                (),
            )
            .unwrap();

        Db { path, connection }
    }

    // let id = db.create_item("new item") guessing so far
    pub fn create_item(&mut self, name: &str) -> Result<(), CreateItemError> {
        let transaction = self.connection.transaction().unwrap();

        transaction
            .execute("INSERT INTO files(name) VALUES (?1)", [name])
            .unwrap();
        let id = transaction.last_insert_rowid();

        let item_path = self.path.join(id.to_string());
        if item_path.exists() {
            return Err(CreateItemError::ItemExists);
        }

        fs::create_dir(item_path).unwrap();

        transaction.commit().unwrap();
        Ok(())
    }

    pub fn fs_root(&self) -> &Path {
        &self.path
    }

    pub fn iterate_items(&self) -> impl Iterator<Item = DbItem> + '_ {
        let mut statement = self
            .connection
            .prepare("SELECT id, name FROM files")
            .unwrap();
        let rows: Vec<_> = statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(DbItem {
                    path: self.path.join(id.to_string()),
                    name: row.get(1)?,
                })
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect();
        rows.into_iter()
    }
}
