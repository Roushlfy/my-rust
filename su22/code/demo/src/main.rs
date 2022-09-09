use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("start", "value");

    let mut start: &str = "Unknown";
    let mut repeat: &str;
    let mut remaining: &str;
    let mut message: &str;

    for key in map.keys() {
        match &key[..] {
            "start" => start = map.get(key).unwrap(),
            "repeat" => repeat = map.get(key).unwrap(),
            "remaining" => remaining = map.get(key).unwrap(),
            "message" => message = map.get(key).unwrap(),
            _ => unreachable!()
        }
    }

    println!("Result: {}", start);
}