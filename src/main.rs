extern crate epub_builder;
use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::Result;
use epub_builder::ZipLibrary;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::fs::File;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::io::prelude::*;
use std::io;
use stdio_override::StdoutOverride;
use yaml_rust::YamlLoader;


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

fn create_epub(title: String, content: String) -> Result<()> {
    // Some dummy content to fill our books
    let dummy_css = "body { background-color: white }";

    let file_name = "./foo.epub";
    let guard = StdoutOverride::override_file(file_name).unwrap();
    EpubBuilder::new(ZipLibrary::new()?)?
        .metadata("author", "web2epub")?
        .metadata("title", title.clone())?
        .stylesheet(dummy_css.as_bytes())?
        .add_content(EpubContent::new(format!("{}.xhtml", title), content.as_bytes())
                     .title(title)
                     .reftype(ReferenceType::Text))?
    // Generate a toc inside of the document, that will be part of the linear structure.
    //    .inline_toc()
        .generate(&mut io::stdout())?;
    drop(guard);
    Ok(())
}

fn load_file(filename: &str) {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut filecontents = String::new();

    file.read_to_string(&mut filecontents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&filecontents).unwrap();

    for array in docs {
        for doc in array {
            let mut content = "".to_string();
            let main_title = doc["title"].clone().into_string().unwrap();
            let outputfolder = format!("./{}/{}", RESULT_PATH, main_title.replace(" ", "_"));
            fs::create_dir_all(outputfolder.clone());
            match fs::remove_file(format!("{}/index.html", outputfolder)){
                Ok(_) => println!("INFO: we are overwriting {}/index.html", outputfolder),
                Err(_) => (),
            };
            //let mut outputfile = OpenOptions::new().append(true).create(true).open(format!("{}/index.html", outputfolder)).unwrap();
            for item in doc["items"].clone() {
                let item_title = item["title"].clone().into_string().unwrap_or("".to_string());
                let url = item["url"].clone().into_string();
                let divs_in = item["divs_in"].clone().into_vec().unwrap();
                let divs_out = item["divs_out"].clone().into_vec().unwrap();
                //for u in url.unwrap().split(" ").collect::<Vec<&str>>() {
                let content_got = get_content(&url.unwrap(), divs_in.clone());
                let content_clean = remove_content(content_got, divs_out.clone());
                content.push_str(&format!("<h1>{}</h1>", item_title));
                content.push_str(&format!("{}", content_clean));
                content.push_str(&format!("<br><br><br>"));
                //create_epub(main_title.clone(), content.clone());
            }
            create_epub(main_title.clone(), content.clone());
        }
    }
}

fn main() {
    let file = "config.yaml";
    load_file(file);
}
