mod schema;
mod manager;
mod errors;

use crate::schema::{Status, Task};
use crate::manager::{TaskManager};

fn main() {
    
    let mut m1 = TaskManager::create();
    println!("New manager created successfully");

    let t1 = m1.add(String::from("I have to go to the market"));
    let t1_res = match t1 {
        Ok(id) => id,
        Err(error) => panic!("Problem while adding talk to list: {error:?}"),
    };
    // println!("{t1_res}");

    
    let t2 = m1.add(String::from("Buy a Laptop"));
    let t2_res = match t2 {
        Ok(id) => id,
        Err(error) => panic!("Problem while adding talk to list: {error:?}"),
    };
    // println!("{t2_res}");


    let t3 = m1.add(String::from("Go to Hell"));
    let t3_res = match t3 {
        Ok(id) => id,
        Err(error) => panic!("Problem while adding talk to list: {error:?}"),
    };
    // println!("{t3_res}");


    let v = m1.list();

    for t in &v {
        println!("{} | {:?} | {}", t.id, t.status, t.task);
    }

    // println!("{:#?}", m1);
}
