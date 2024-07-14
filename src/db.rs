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
pub enum RelationshipSide {
    Source,
    Dest,
}

#[derive(Debug)]
pub struct Relationship {
    pub id: RelationshipId,
    pub side: RelationshipSide,
}

#[derive(Debug)]
pub struct RelationshipId(i64);

#[derive(Debug)]
pub struct ItemId(pub i64);

#[derive(Debug)]
pub struct Db {
    item_path: PathBuf,
    connection: Connection,
}

#[derive(Debug)]
pub struct DbItem {
    // Our Db item is gonna have path to the item and his name and id too
    pub path: PathBuf,
    pub id: ItemId,
    pub relationships: Vec<Relationship>,
    pub name: String,
}

impl Db {
    pub fn new(path: PathBuf) -> Db {
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        let sqlite_path = path.join("metadata.db");
        let connection = Connection::open(sqlite_path).unwrap();

        // parent - child
        // blocks - blocked by relationship
        // other relationships depending on the requirements
        // relationships(id, from, to)
        // 1 , parents, children
        // 2, blocked, blocked by
        //
        // items relationships (from_id, to_id, relationship)
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS files(id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                (),
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS relationships(id INTEGER PRIMARY KEY, from_name TEXT NOT NULL, to_name TEXT NOT NULL)",
                (),
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS item_relationships(from_id INTEGER, to_id INTEGER, relationship_id INTEGER,
                FOREIGN KEY(from_id) REFERENCES files(id),
                FOREIGN KEY(to_id) REFERENCES files(id),
                FOREIGN KEY(relationship_id) REFERENCES relationships(id))",
                (),
            )
            .unwrap();

        let item_path = path.join("items");
        Db {
            item_path,
            connection,
        }
    }

    // let id = db.create_item("new item") guessing so far
    pub fn create_item(&mut self, name: &str) -> Result<(), CreateItemError> {
        let transaction = self.connection.transaction().unwrap();

        transaction
            .execute("INSERT INTO files(name) VALUES (?1)", [name])
            .unwrap();
        let id = transaction.last_insert_rowid();

        let item_path = self.item_path.join(id.to_string());
        if item_path.exists() {
            return Err(CreateItemError::ItemExists);
        }

        fs::create_dir(item_path).unwrap();

        transaction.commit().unwrap();
        Ok(())
    }

    pub fn add_relationship(&mut self, from_name: &str, to_name: &str) -> RelationshipId {
        if let Some(id) = self.find_relationship(from_name) {
            log::warn!(
                "relationship with from_name \"{:?}\" already exists",
                from_name
            );
            return id;
        }

        let transaction = self.connection.transaction().unwrap();
        transaction
            .execute(
                "INSERT INTO relationships(from_name, to_name) VALUES (?1, ?2)",
                [from_name, to_name],
            )
            .unwrap();
        let id = transaction.last_insert_rowid();
        transaction.commit().unwrap();
        RelationshipId(id)
    }

    pub fn find_relationship(&mut self, from_name: &str) -> Option<RelationshipId> {
        let mut statement = self
            .connection
            .prepare("SELECT id FROM relationships WHERE from_name = ?1")
            .unwrap();

        let item = statement
            .query_map([from_name], |row| {
                let ret: i64 = row.get(0)?;
                Ok(RelationshipId(ret))
            })
            .unwrap()
            .next();
        // TODO: what is there are duplicates

        item.map(|item| item.unwrap())
    }

    pub fn add_item_relationship(
        &mut self,
        from_id: ItemId,
        to_id: ItemId,
        relationship_id: RelationshipId,
    ) {
        let transaction = self.connection.transaction().unwrap();
        transaction
            .execute(
                "INSERT INTO item_relationships(from_id, to_id, relationship_id) VALUES (?1, ?2, ?3)",
                [from_id.0, to_id.0, relationship_id.0],
            )
            .unwrap();

        transaction.commit();
    }

    pub fn fs_root(&self) -> &Path {
        &self.item_path
    }

    pub fn iterate_items(&self) -> impl Iterator<Item = DbItem> + '_ {
        let mut statement = self
            .connection
            .prepare("SELECT id, name FROM files")
            .unwrap();
        let rows: Vec<_> = statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let id = ItemId(id);
                Ok(DbItem {
                    path: self.item_path.join(id.0.to_string()),
                    id,
                    name: row.get(1)?,
                })
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect();
        rows.into_iter()
    }
}
