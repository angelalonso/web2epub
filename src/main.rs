use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::fs::OpenOptions;
use std::io::prelude::*;

static RESULT_PATH: &str = "ebooks";

fn get_content(url: &str, divs_in: Vec<yaml_rust::Yaml>) -> String {
    let mut result = "".to_string();
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    for d_i in divs_in {
        for (k, v) in d_i.as_hash().unwrap().iter() {
            let key = k.as_str().unwrap();
            let val = v.as_str().unwrap();
            if key == "class" {
                for node in document.find(Class(val)) {
                    result.push_str(&format!("{}", node.inner_html()));
                }
            } else if key == "id" {
                for node in document.find(Attr(key, val)) {
                    result.push_str(&format!("{}", node.inner_html()));
                }
            };
        }
    }
    return result
}

fn remove_content(mut content: String, divs_out: Vec<yaml_rust::Yaml>) -> String{
    let mut remover = "".to_string();
    let document = Document::from_read(content.as_bytes()).unwrap();
    for d_o in divs_out {
        for (k, v) in d_o.as_hash().unwrap().iter() {
            let key = k.as_str().unwrap();
            let val = v.as_str().unwrap();
            if key == "class" {
                for node in document.find(Class(val)) {
                    content = content.replace(&node.inner_html(), "");
                }
            } else if key == "id" {
                for node in document.find(Attr(key, val)) {
                    content = content.replace(&node.inner_html(), "");
                }
            };
        }
    }
    return content
}

fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    for array in docs {
        for doc in array {
            let main_title = doc["title"].clone().into_string().unwrap();
            let outputfolder = format!("./{}/{}", RESULT_PATH, main_title.replace(" ", "_"));
            fs::create_dir_all(outputfolder.clone());
            match fs::remove_file(format!("{}/index.html", outputfolder)){
                Ok(_) => println!("INFO: we are overwriting {}/index.html", outputfolder),
                Err(_) => (),
            };
            let mut outputfile = OpenOptions::new().append(true).create(true).open(format!("{}/index.html", outputfolder)).unwrap();
            for item in doc["items"].clone() {
                let item_title = item["title"].clone().into_string().unwrap_or("".to_string());
                let url = item["url"].clone().into_string();
                let divs_in = item["divs_in"].clone().into_vec().unwrap();
                let divs_out = item["divs_out"].clone().into_vec().unwrap();
                //for u in url.unwrap().split(" ").collect::<Vec<&str>>() {
                let content_got = get_content(&url.unwrap(), divs_in.clone());
                let content_clean = remove_content(content_got, divs_out.clone());
                write!(&mut outputfile, "{}", format!("<h1>{}</h1>", item_title));
                write!(&mut outputfile, "{}", format!("{}", content_clean));
                write!(&mut outputfile, "{}", format!("<br><br><br>"));
                //println!("<h1>{}</h1>", item_title);
                //println!("{}", content_clean);
                //println!("<br><br><br>");
                //}
            }
        }
    }
}

fn main() {
    let file = "config.yaml";
    load_file(file);
}
