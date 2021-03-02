use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

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
            let urls = doc["urls"].clone().into_string();
            for u in urls.unwrap().split(" ").collect::<Vec<&str>>() {
                // TODO: download documents into memory
                println!("{:?}", u);

            }
        }
    }
}
