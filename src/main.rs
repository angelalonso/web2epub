extern crate epub_builder;
use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::Result;
use epub_builder::ZipLibrary;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use web2epub::correctors::{get_images, do_calibre_autocorrect};


static RESULT_FOLDER: &str = "ebooks";
static HTML_FOLDER: &str = "tmphtml";

fn create_from_cfg_file(filename: &str) {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut filecontents = String::new();

    file.read_to_string(&mut filecontents)
        .expect("Unable to read file");

    let docs = match YamlLoader::load_from_str(&filecontents) {
        Ok(d) => d,
        Err(e) => {
            println!("ERROR parsing yaml: {}", e);
            panic!();
        },
    };

    for array in docs {
        for doc in array {
            //let mut content = "".to_string();
            let mut titles: Vec<String> = [].to_vec();
            let mut contents: Vec<String> = [].to_vec();
            let mut images: Vec<String> = [].to_vec();
            let main_title = doc["title"].clone().into_string().unwrap();
            for item in doc["items"].clone() {
                let item_title = item["title"].clone().into_string().unwrap_or_else(|| "".to_string());
                let url = item["url"].clone().into_string().unwrap();
                let divs_in = match item["divs_in"].clone().into_vec() {
                    Some(d_i) => d_i,
                    _ => [].to_vec(),
                };
                let divs_out = match item["divs_out"].clone().into_vec() {
                    Some(d_o) => d_o,
                    _ => [].to_vec(),
                };
                let content_got = get_content(&url.clone(), divs_in.clone());
                let content_clean = remove_content(content_got, divs_out.clone());
                let mut content = "".to_string();
                content.push_str("<?xml version='1.0' encoding='utf-8'?><html xmlns=\"http://www.w3.org/1999/xhtml\"><head/><body>");
                content.push_str(&format!("<h1>{}</h1>", item_title));
                content.push_str(&"<br><br><br>".to_string());
                let (mut aux_images, aux_content) = get_images(content_clean.clone(), url.clone());
                content.push_str(&aux_content);
                titles.append(&mut [item_title].to_vec());
                contents.append(&mut [content].to_vec());
                images.append(&mut aux_images);
            }
            if is_update_needed(main_title.clone(), contents.clone()) {
                match create_epub(main_title.clone(), titles.clone(), contents.clone(), images.clone()) {
                    Ok(_) => println!("Book {}.epub created successfully!\n\nUse ebook-edit {}.epub > Tools > Check ebook if your ebook reader cant handle it", 
                                      main_title.clone().replace(" ", "_"),
                                      main_title.clone().replace(" ", "_")),
                    Err(_) => println!("ERROR creating Book {}.epub!", main_title.clone().replace(" ", "_")),
                };
                do_calibre_autocorrect(&main_title.clone());
            };
        }
    }
}

fn get_content(url: &str, divs_in: Vec<yaml_rust::Yaml>) -> String {
    println!(" - downloading {}", url.clone());
    let mut result = "".to_string();
    let resp = reqwest::blocking::get(url).unwrap();
    if !resp.status().is_success() {
        println!("ERROR downloading \"{}\"", url);
        panic!();
    };

    let document = Document::from_read(resp).unwrap();
    for d_i in divs_in {
        for (k, v) in d_i.as_hash().unwrap().iter() {
            let key = k.as_str().unwrap();
            let val = v.as_str().unwrap();
            if key == "class" {
                for node in document.find(Class(val)) {
                    result.push_str(&node.inner_html().to_string());
                }
            } else if key == "id" {
                for node in document.find(Attr(key, val)) {
                    result.push_str(&node.inner_html().to_string());
                }
            };
        }
    }
    result
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
    content
}

fn is_update_needed(title: String, contents: Vec<String>) -> bool {
    //if let Ok(_) = fs::create_dir_all(HTML_FOLDER) { () };
    if fs::create_dir_all(HTML_FOLDER).is_ok() { };
    let mut result = true;
    let file_name = format!("{}/{}.html", HTML_FOLDER, title.replace(" ", "_"));
    let content = contents.join(" ");
    if fs::metadata(file_name.clone()).is_ok() {
        let mut prev_content = String::new();
        let mut prev_f = File::open(file_name.clone()).expect("Unable to open file");
        prev_f.read_to_string(&mut prev_content).expect("Unable to read string");
        if content == prev_content {
            println!("- '{}' is up to date", title);
            result = false;
        } else {
            println!("- Updating '{}'...", title);
            let mut f = File::create(file_name).expect("Unable to create file");
            f.write_all(content.as_bytes()).expect("Unable to write data");
        }
    } else {
        println!("- Adding '{}'...", title);
        let mut f = File::create(file_name).expect("Unable to create file");
        f.write_all(content.as_bytes()).expect("Unable to write data");
    }
    result
}

fn create_epub(main_title: String, titles: Vec<String>, contents: Vec<String>, images: Vec<String>) -> Result<()> {

    let file_name = format!("{}/{}.epub", RESULT_FOLDER, main_title.replace(" ", "_"));
    if fs::remove_file(file_name.clone()).is_ok()  { };
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    let imgs = images.iter().map(|x| fs::read(x).expect("Unable to read file")).collect::<Vec<Vec<u8>>>();

    let mut book = EpubBuilder::new(ZipLibrary::new()?)?;
    book.metadata("author", "web2epub")?
        .metadata("title", main_title.clone())?
        .inline_toc();
    for i in 0..contents.len() {
        book.add_content(EpubContent::new(format!("{}.xhtml", titles[i]), contents[i].as_bytes())
                     .title(titles[i].clone())
                     .reftype(ReferenceType::Text))?;
    }
    for i in 0..images.len() {
        book.add_resource(images[i].clone(), imgs[i].as_slice(), "image/png")?;
    }

    book.generate(&mut f)?;
    Ok(())
}

fn main() {
    let cfg_file = "config.yaml";
    // TODO:
    //    - Add --force mode to force update even if nothing changed
    //    - Check calibre changes and do them here
    create_from_cfg_file(cfg_file);
}
