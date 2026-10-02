mod schema;
mod manager;

use crate::schema::{Status, Task};
use crate::manager::{TaskManager};

fn main() {
    let t1 = Task {
        status : Status::Done, 
        id : 123, 
        task : String::from("bring me a glass of water"),
    };

    println!("{:?}", t1);

    dbg!(&t1);

    let m1 = TaskManager::create();

    println!("New manager created successfully")
}
