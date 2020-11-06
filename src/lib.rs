use serde_json::Value;
use std::io::BufReader;
use std::fs::File;

const BASEURL: &str = "https://nekos.life/api/v2";

pub fn nsfw(text: &str) -> std::string::String {
    if json("nsfw")[text].is_null() {
        return "NOT FOUND".to_string();
    } else {
        let resp = ureq::get(&(BASEURL.to_owned() + &json("nsfw")[text].to_string().replace("\\", "").replace("\"", "")))
            .call();
        let url: Value = resp.into_json().unwrap();
        return url["url"].to_string();
    } 
}

pub fn sfw(text: &str) -> std::string::String {
    if json("sfw")[text].is_null() {
        return "NOT FOUND".to_string();
    } else {
        let resp = ureq::get(&(BASEURL.to_owned() + &json("sfw")[text].to_string().replace("\\", "").replace("\"", "")))
            .call();
        let url: Value = resp.into_json().unwrap();
        return url["url"].to_string();
    } 
}

fn json(url: &str) -> serde_json::Value {
    let file = File::open("./src/endpoints.json").unwrap();
    let reader = BufReader::new(file);

    let read: Value = serde_json::from_reader(reader).unwrap();
    let value = serde_json::from_str(&read[url].to_string()).unwrap();
    return value;
}
