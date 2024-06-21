use db::Db;
mod db;
mod fuse;

fn main() {
    let db_path = "test_db";
    let mut db = Db::new(db_path.into());
    db.create_item("test").unwrap();
    db.create_item("test2").unwrap();

    fuse::run_fuse_client();
}
