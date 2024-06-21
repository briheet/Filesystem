use db::Db;
mod db;
mod fuse;

fn main() {
    let db_path = "test_db";
    let db = Db::new(db_path.into());

    fuse::run_fuse_client(db);
}
