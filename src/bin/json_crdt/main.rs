mod jsondoc;
mod op;
mod interconnect;

use serde_json::{json, Result, Value};
use crate::jsondoc::JSONDoc;
use tokio::sync::broadcast;
use tokio::select;
use std::thread;


/**
1. all the participants join at once, and then we start executing
2. all participants would be on different threads
3. use tokio as the async runtime for handling the communication between different JSONDoc
instances, spawn runtimes for json doc clients
    1. tokio runtime spawning on the same thread of json worker
    2. runtime responsible for sending operations and receiving operations
    3. use Arc to share all the data
*/

fn main() {
    // untyped_example().unwrap();
    // for every thread that we want to create, we create a json document
    // the json document will apply the changes to themselves, and then dispatch
    // those to all other clients
    // let a = JSONDoc::new(1);
    // println!("{}", a);

}

// #[tokio::main]
// async fn main() {
//     // Create a broadcast channel
//     let (tx, _) = broadcast::channel(16);
//
//     let tx2 = tx.clone();
//
//     // Create multiple receivers
//     let mut rx1 = tx.subscribe();
//     let mut rx2 = tx.subscribe();
//
//     // Send data asynchronously
//     tokio::spawn(async move {
//         for i in 0..10 {
//             tx.send(i).unwrap();
//             tx2.send(-i).unwrap();
//             tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//         }
//     });
//
//     loop {
//         select! {
//             result = rx1.recv() => {
//                 match result {
//                     Ok(message) => println!("Receiver 1 got: {}", message),
//                     Err(e) => {break;},
//                 }
//             },
//             result = rx2.recv() => {
//                 match result {
//                     Ok(message) => println!("Receiver 2 got: {}", message),
//                     Err(e) => {break;},
//                 }
//             },
//             else => break,
//         }
//     }
// }


// #[tokio::main]
// async fn main() {
//     // Create a broadcast channel
//     let (tx, _rx) = broadcast::channel(10);
//
//     // Spawn a task that sends messages
//     let tx_clone = tx.clone();
//     tokio::spawn(async move {
//         loop {
//             tx_clone.send("message").unwrap();
//             tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//         }
//     });
//
//     // Spawn multiple receiver tasks
//     for _ in 0..5 {
//         let mut rx = tx.subscribe();
//         tokio::spawn(async move {
//             loop {
//                 select! {
//                     Ok(msg) = rx.recv() => {
//                         println!("Received: {}", msg);
//                     }
//                     // You can listen to other events here
//                 }
//             }
//         });
//     }
//
//     // Keep the main task alive for demonstration purposes
//     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
// }


fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
  {
      "name": "John Doe",
      "age": 30,
      "phones": ["123-4567", "555-0123"]
  }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();

    // Access and modify the data
    v["age"] = json!(31); // Change age to 31
    v["phones"][0] = json!("987-6543"); // Change first phone number

    println!("{}", v.to_string());

    Ok(())
}
