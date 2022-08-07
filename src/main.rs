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

fn main() {
    println!("Hello, world!");

    for a in args() {
        println!("arg: {}", a);
        if a.starts_with("https:") {
            let body = curl(a);

            //println!("body = {:?}", body);
            match json::parse(&body){
                Ok(j) => println!("{:?}", j),
                Err(e) => println!("ERROR: Not json: {:?}", e)
            }
        }
    }
}
