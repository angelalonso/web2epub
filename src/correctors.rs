use std::fs::File;
use std::io;

#[allow(dead_code)]
pub fn do_correct_images() -> &'static str {
    "test"
}

pub fn get_images(content: String, source_url: String) -> (Vec<String>, String) {
    let mut result_content: String = "".to_string();
    let mut result_imgs: Vec<String> = [].to_vec();
    let img_list = get_from_string(content.clone(), "<img", ">");
    for img in img_list.clone() {
        let src_list = get_from_string(img.clone(), "src=", "\"");
        for src in src_list {
            let url: String;
            //println!("{}", src);
            if src.contains("http") {
                url = src.replace("src=", "")
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
                url = source_url.clone().replace(source_url_file, "") + "/" + &url_part;
            }
            let url_decomp = url.split("/").collect::<Vec<_>>();
            // TODO: check if the filename exists and rename it to *_00x.* if needed
            let file_name = "images/".to_owned() + url_decomp[url_decomp.len() - 1];
            result_imgs.append(&mut [file_name.clone()].to_vec());
            let mut resp = reqwest::get(&url).expect("request failed");
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
