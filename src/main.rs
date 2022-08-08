use std::env::args;

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
                let p = format!("{}.{}", path, k);
                walk_json(&p, v.to_owned());
            }
        }
        json::JsonValue::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                let p = format!("{}[{}]", path, i);
                walk_json(&p, v.to_owned());
            }
        }
    }
}

fn main() -> Result<(), json::Error> {
    for a in args() {
        println!("arg: {}", a);
        if a.starts_with("https:") {
            let body = match curl(a) {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e);
                    String::from("")
                }
            };

            let j = json::parse(&body).unwrap();
            walk_json("", j);
        }
    }

    Ok(())
}
