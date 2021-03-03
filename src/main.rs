use std::fs::File;
use std::io;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn get_content(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    for node in document.find(Class("td-content")) {
    //for node in document.find(Attr("id", "content")) {
        println!("{:#x?}", node.inner_html());
    }
}

fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    for array in docs {
        for doc in array {
            println!("{:?}", doc["title"].clone().into_string());
            let title = doc["title"].clone().into_string().unwrap();
            println!("{:?}", title);
            let url = doc["url"].clone().into_string();
            for u in url.unwrap().split(" ").collect::<Vec<&str>>() {
                get_content(u);
            }
        }
    }
}

fn main() {
    let file = "config.yaml";
    load_file(file);
}
