use bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};
//
fn main() {
    //connect with mongodb
    let client= Client::with_uri_str("mongodb+srv://makkk:<password>@cluster0.wrgd4.mongodb.net/testretryWrites=true&w=majority").expect("Failed to connect");
    //make database and a collection
    let db = client.database("test123").collection("user_details");
    // inserting a document
    let docs = doc! { "user_name": "mak123" };
    let data = db.insert_one(docs, None).unwrap();
    println!("id is : {:#?}", data);
    // find a document
    let docs1 = doc! { "user_name": "mak123" };
    let data = db.find_one(docs1,None).unwrap();
    match data {
        Some(data) => {
            let data: Value = json!(data);
            println!("data is : {}", data);
        }
        None => println!("No record Found"),
    }

    //updating a doc
    let filter = doc! { "user_name": "mak123" };
    let replacement = doc! { "user_name": "aqib123" };
    db.find_one_and_replace(filter,replacement,None);

    //for confirming
    let docs1 = doc! { "user_name": "aqib123" };
    let data = db.find_one(docs1,None).unwrap();
    match data {
        Some(data) => {
            let data: Value = json!(data);
            println!("data is : {}", data["user_name"]);
        }
        None => println!("No record Found"),
    }


    // //deleting a document
    // let query = doc! { "user_name": "mak123" };
    //  db.delete_one(query,None);
    // //for confirming
    // let docs2 = doc! { "user_name": "mak123" };
    // let data = db.find_one(docs2,None).unwrap();
    // match data {
    //     Some(data) => {
    //         let data: Value = json!(data);
    //         println!("data is : {}", data["user_name"]);
    //     }
    //     None => println!("No record Found"),
    // }

    
}
