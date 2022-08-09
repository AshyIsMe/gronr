use std::env::args;
use std::fs;

fn curl(url: String) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

fn walk_json(path: &str, j: json::JsonValue) {
    match j {
        json::JsonValue::Null => println!("{}: null", path),
        json::JsonValue::Short(s) => println!("{}: {}", path, s),
        json::JsonValue::String(s) => println!("{}: {}", path, s),
        json::JsonValue::Number(n) => println!("{}: {}", path, n),
        json::JsonValue::Boolean(b) => println!("{}: {}", path, b),
        json::JsonValue::Object(o) => {
            for (k, v) in o.iter() {
                walk_json(&format!("{}.{}", path, k), v.to_owned());
            }
        }
        json::JsonValue::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                walk_json(&format!("{}[{}]", path, i), v.to_owned());
            }
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    for a in args().skip(1) {
        println!("arg: {}", a);
        if a.starts_with("https:") {
            let body = curl(a)?;
            walk_json("", json::parse(&body).unwrap());
        } else {
            match fs::read_to_string(&a) {
                Ok(s) => walk_json("", json::parse(&s).unwrap()),
                Err(e) => panic!("couldn't read {}: {}", a, e),
            }
        }
    }

    Ok(())
}
