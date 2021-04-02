use std::fs::File;
use std::path::Path;
use std::io;
use std::process::Command;
use reqwest;

static CALIBRE_EDIT_COMMAND: &str = "ebook-edit";
static RESULT_FOLDER: &str = "ebooks";

pub fn do_calibre_autocorrect(title: &str) {
    Command::new(CALIBRE_EDIT_COMMAND)
        .arg(format!("{}/{}.epub", RESULT_FOLDER, title.replace(" ", "_")))
        .spawn()
        .expect("ebook-edit command failed to start");
}

pub fn get_image_filename(url: String) -> String {
    let path = "images/".to_owned();
    let url_decomp = url.split("/").collect::<Vec<_>>();
    let stripped_file_name_raw = url_decomp[url_decomp.len() - 1].split(".").collect::<Vec<_>>();
    let stripped_file_name = stripped_file_name_raw.split_last().unwrap_or((&"", &[""]));
    let mut file_name = path.clone() + url_decomp[url_decomp.len() - 1];
    if Path::new(&file_name).exists() {
        let mut name_correct = false;
        let mut file_ix = 0;
        while !name_correct {
            file_ix += 1;
            let test_file_name = path.clone() 
                + &stripped_file_name.1.join(".") 
                + &format!("{:0>3}", file_ix) 
                + "." + stripped_file_name.0;
            if !Path::new(&test_file_name).exists() {
                file_name = test_file_name;
                name_correct = true;
            }
        }
    };
    file_name
}

pub fn get_images(content: String, source_url: String) -> (Vec<String>, String) {
    let mut result_content: String = "".to_string();
    let mut result_imgs: Vec<String> = [].to_vec();
    let img_list = get_from_string(content.clone(), "<img", ">");
    for img in img_list.clone() {
        let src_list = get_from_string(img.clone(), "src=", "\"");
        for src in src_list {
            let img_url: String;
            if src.contains("http") {
                img_url = src.replace("src=", "")
                    .replace("src =", "")
                    .replace("\"", "")
                    .replace(" ", "");
            } else {
                let url_part = src.replace("src=", "")
                    .replace("src =", "")
                    .replace("\"", "")
                    .replace(" ", "");
                let source_url_decomp = source_url.split("/").collect::<Vec<_>>();
                let source_url_file = source_url_decomp[source_url_decomp.len() - 1];
                img_url = source_url.clone().replace(source_url_file, "") + "/" + &url_part;
            }
            println!("--------------- {}", img_url);
            let file_name = get_image_filename(img_url.clone());
            result_imgs.append(&mut [file_name.clone()].to_vec());
            let mut resp = reqwest::blocking::get(&img_url).expect("request failed");
            let mut out = File::create(file_name.clone()).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy content");
            result_content = content.clone().replace(&img.clone(), &file_name.clone());
        }
    }
    (result_imgs, result_content)

}

/// Gets the exact parts of a String fitting a pattern
/// First use case: find an image definition in HTML
pub fn get_from_string(content: String, search_start: &str, search_end: &str) -> Vec<String> {
    let mut result = [].to_vec();
    let starts: Vec<_> = content.match_indices(search_start).collect();
    let ends: Vec<_> = content.match_indices(search_end).collect();
    let mut pos = [].to_vec();
    for s in starts {
        let mut p = [s.0].to_vec();
        let mut e_found = false;
        let mut ix = 0;
        while !e_found && ix < ends.len() {
            if ends[ix].0 > s.0 + search_start.len(){
                e_found = true;
                p.append(&mut [ends[ix].0].to_vec());
            }
            ix += 1;
        }
        pos.append(&mut [p].to_vec());
        //println!("{:?}", pos);
    }
    for st in pos {
        result.append(&mut [format!("{}", &content[st[0]..st[1]+1])].to_vec())
    }
    result
    
}
