extern crate epub_builder;
use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::Result;
use epub_builder::ZipLibrary;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::process::Command;


fn do_calibre_autocorrect(title: &str) {
    Command::new("ebook-edit")
        .arg(format!("ebooks/{}.epub", title.replace(" ", "_")))
        .spawn()
        .expect("ebook-edit command failed to start");
}


fn create_from_cfg_file(filename: &str) {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut filecontents = String::new();

    file.read_to_string(&mut filecontents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&filecontents).unwrap();

    for array in docs {
        for doc in array {
            let mut content = "".to_string();
            let main_title = doc["title"].clone().into_string().unwrap();
            for item in doc["items"].clone() {
                let item_title = item["title"].clone().into_string().unwrap_or("".to_string());
                let url = item["url"].clone().into_string();
                let divs_in = item["divs_in"].clone().into_vec().unwrap();
                let divs_out = item["divs_out"].clone().into_vec().unwrap();
                let content_got = get_content(&url.unwrap(), divs_in.clone());
                let content_clean = remove_content(content_got, divs_out.clone());
                content.push_str("<?xml version='1.0' encoding='utf-8'?><html xmlns=\"http://www.w3.org/1999/xhtml\"><head/><body>");
                content.push_str(&format!("<h1>{}</h1>", item_title));
                content.push_str(&format!("{}", content_clean));
                content.push_str(&format!("<br><br><br>"));
            }
            match create_epub(main_title.clone(), content.clone()) {
                Ok(_) => println!("Book {}.epub created successfully!\n\nUse ebook-edit {}.epub > Tools > Check ebook if your ebook reader cant handle it", 
                                  main_title.clone().replace(" ", "_"),
                                  main_title.clone().replace(" ", "_")),
                Err(_) => println!("ERROR creating Book {}.epub!", main_title.clone().replace(" ", "_")),
            };
            do_calibre_autocorrect(&main_title.clone());
        }
    }
}

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

fn create_epub(title: String, content: String) -> Result<()> {

    let file_name = format!("ebooks/{}.epub",title.replace(" ", "_"));
    match fs::remove_file(file_name.clone()) {
        Ok(_) => (),
        Err(_) => (),
    };
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    EpubBuilder::new(ZipLibrary::new()?)?
        .metadata("author", "web2epub")?
        .metadata("title", title.clone())?
        .add_content(EpubContent::new(format!("{}.xhtml", title), content.as_bytes())
                     .title(title)
                     .reftype(ReferenceType::Text))?
    // Use this if we want to generate a toc inside of the document.
    //    .inline_toc()
        .generate(&mut f)?;
    Ok(())
}

fn main() {
    let cfg_file = "config.yaml";
    create_from_cfg_file(cfg_file);
}
