use std::fs::File;
use std::io;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn get_content(url: &str, divs_in: Vec<yaml_rust::Yaml>) -> String {
    let mut result = "".to_string();
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    for node in document.find(Class("td-content")) {
    //for node in document.find(Attr("id", "content")) {
        result.push_str(&format!("{:#x?}", node.inner_html()));
    }
    for d_i in divs_in {
        for (k, v) in d_i.as_hash().unwrap().iter() {
            let key = k.as_str().unwrap();
            let val = v.as_str().unwrap();
            if key == "class" {
                for node in document.find(Class(val)) {
                    result.push_str(&format!("{:#x?}", node.inner_html()));
                }
            } else if key == "id" {
                for node in document.find(Attr(key, val)) {
                    println!("{:#x?}", node.inner_html());
                    result.push_str(&format!("{:#x?}", node.inner_html()));
                }
            };
        }
    }
    return result
}

fn remove_content(content: String, divs_out: Vec<yaml_rust::Yaml>) -> String{
    let mut result = "".to_string();
    let document = Document::from_read(content.as_bytes()).unwrap();
    for d_o in divs_out {
        for (k, v) in d_o.as_hash().unwrap().iter() {
            let key = k.as_str().unwrap();
            let val = v.as_str().unwrap();
            if key == "class" {
                for node in document.find(Class(val)) {
                    result.push_str(&format!("{:#x?}", node.inner_html()));
                }
            } else if key == "id" {
                for node in document.find(Attr(key, val)) {
                    println!("{:#x?}", node.inner_html());
                    result.push_str(&format!("{:#x?}", node.inner_html()));
                }
            };
        }
    }
    return result
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
            let divs_in = doc["divs_in"].clone().into_vec().unwrap();
            let divs_out = doc["divs_out"].clone().into_vec().unwrap();
            for u in url.unwrap().split(" ").collect::<Vec<&str>>() {
                let content_got = get_content(u, divs_in.clone());
                println!("{:?}", content_got);
                let content_clean = remove_content(content_got, divs_out.clone());
                //println!("{:?}", content_clean);
            }
        }
    }
}

fn main() {
    let file = "config.yaml";
    load_file(file);
}
