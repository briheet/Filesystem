use db::Db;
use std::path::Path;
mod db;
mod fuse;

fn main() {
    let db_path = std::env::args().nth(1).unwrap();
    let mut db = Db::new(db_path.into());
    db.create_item("test").unwrap();
    db.create_item("test2").unwrap();

    let fuse_args = println!("{:?}", db);
}
