use serde_json::{json, Value};
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("integer".to_string(), json!(42));
    map.insert("float".to_string(), json!(3.14));
    map.insert("string".to_string(), json!("Hello, Rust!"));
    map.insert("empty".to_string(), Value::Null);

    println!("map = {:?}", map);
    let json_str = serde_json::to_string(&map).unwrap();
    println!("JSON: {}", json_str);
}
