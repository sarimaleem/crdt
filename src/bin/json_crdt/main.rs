mod op;

use serde_json::{json, Result, Value};

fn main() {
    untyped_example().unwrap();
}

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
