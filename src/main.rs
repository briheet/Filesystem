use db::Db;
mod db;
mod fuse;

fn main() {
    let db_path = "test_db";
    let mut db = Db::new(db_path.into());

    println!("{:?}", db.find_relationship("blocks"));

    fuse::run_fuse_client(db);
}
