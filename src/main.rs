use db::Db;
use std::path::Path;
mod db;

fn main() {
    let db_path = std::env::args().nth(1).unwrap();
    let db = Db::new(db_path.into());
    println!("{:?}", db);
}
