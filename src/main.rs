use std::fs::File;
use std::io;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use reqwest;
use kuchiki;
use kuchiki::traits::*;

fn main() {
    let file = "config.yaml";
    load_file(file);
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
            let urls = doc["urls"].clone().into_string();
            for u in urls.unwrap().split(" ").collect::<Vec<&str>>() {
                // TODO: download documents into memory
                let mut resp = reqwest::get(u).expect("request failed");
                let filename: String = format!("./ebooks/{}.html", title.replace(" ", "_"));
                let mut out = File::create(filename).expect("failed to create file");
                let result = resp.text().unwrap();
                let document = kuchiki::parse_html().one(result);
                let selector = "div class=td-content";
                let anchor = document.select_first(selector).unwrap();
                // Iterating solution - Using `text_nodes()` iterators
                println!("{:?}", anchor);
                anchor.as_node().children().text_nodes().for_each(|e| {
                    println!("{:?}", e);
                });
                io::copy(&mut resp, &mut out).expect("failed to copy content");

            }
        }
    }
}
