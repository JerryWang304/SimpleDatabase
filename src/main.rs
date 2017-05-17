#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
extern crate simple_db;
//use std::option::Option;
fn test_hashmap() {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("foo","bar");
    match data.get("fo") {
        Some(x) => {
            println!("{}", x);
        },
        None => {
            println!("None");
        }
    }  
}
#[allow(dead_code)]
fn test_in_memory_simpledb() {
    let mut db = simple_db::SimpleDB::new().unwrap();
    db.set("Apple","1");
    db.set("Google","2");
    println!("{:?}", db.get("Apple").unwrap());
    println!("{:?}", db.get("Google").unwrap());
    db.delete("Google");
    //println!("{:?}", db.get("AApple").unwrap());
    println!("{:?}", db.get("Google").unwrap());
}
fn test_in_memory_simple_db_multithreads() {
    let db = simple_db::SimpleDB::new().unwrap();
    // db.set("Apple","1");
    // db.set("Google","2");
    let shared = Arc::new(db);
    let mut threads = Vec::new();
    
    for i in 0..3 {
        let db = shared.clone();
        let names = vec!["Apple","Google","Amazon"];
        threads.push(thread::spawn(move ||{
            let value = i.to_string();
            let name = names[i];
            db.set(name,&value);

        }));
        //assert!(db.get("sgs").is_none());
    }
    // write
    // for _ in 0..1 {
    //     let mut db = shared.clone();
    //     //db.set("Google","1000");

    //     threads.push(thread::spawn(move ||{
    //         db.set("Google","1000");
    //     }));        
    // }
    for t in threads {
        assert!(t.join().is_ok());
    }
    println!("{}", shared.get("Google").unwrap());
}

fn main() {        
    //test_hashmap();
    //assert_eq!(data["fo"], "bar");
    test_in_memory_simple_db_multithreads();
}