use db::{Db, ItemId, RelationshipId};
mod db;
mod fuse;

fn main() {
    env_logger::init();
    let db_path = "test_db";
    let mut db = Db::new(db_path.into());

    println!("{:?}", db.find_relationship("blocks"));
    println!("{:?}", db.find_relationship("blocked-by"));

    let parent_relationship = db.add_relationship("parents", "children");
    db.add_item_relationship(db::ItemId(1), db::ItemId(2), parent_relationship);

    for item in db.iterate_items() {
        println!("{:?}", item);
    }

    fuse::run_fuse_client(db);
}
