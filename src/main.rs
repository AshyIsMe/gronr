use std::env::args;
use reqwest;

fn curl(url: String) -> String {
    let r = match reqwest::blocking::get(url) {
        Ok(r) => r.text(),
        Err(e) => Ok(String::from(""))
    };

    println!("{:?}", r);
    match r {
        Ok(t) => t,
        Err(e) => String::from("")
    }
}

fn main() -> Result<(), json::Error> {

    for a in args() {
        println!("arg: {}", a);
        if a.starts_with("https:") {
            let body = curl(a);

            //println!("body = {:?}", body);
            let j =  json::parse(&body).unwrap();
            //println!("{:?}", j);
            match j {
                json::JsonValue::Object(o) => {
                    for (k, v) in o.iter() {
                        println!("{:?}: {:?}", k, v);
                    }
                }
                _ => println!("j isnt an object: {:?}", j)
            }
        }
    }

    Ok(())
}
