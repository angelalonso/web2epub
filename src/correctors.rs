#[allow(dead_code)]
pub fn do_correct_images() -> &'static str {
    "test"
}

pub fn get_images(content: String, source_url: String) -> Vec<&'static str> {
    let img_list = get_from_string(content, "<img", ">");
    for img in img_list {
        let src_list = get_from_string(img, "src=", "\"");
        for src in src_list {
            let mut url = "".to_string();
            println!("{}", src);
            if src.contains("http") {
                url = src.replace("src=", "")
                    .replace("src =", "")
                    .replace("https://", "")
                    .replace("http://", "")
                    .replace("\"", "")
                    .replace(" ", "");
            } else {
                let url_part = src.replace("src=", "")
                    .replace("src =", "")
                    .replace("\"", "")
                    .replace(" ", "");
                url = source_url.clone() + "/" + &url_part;
            }
            println!("### {}", url);
            //TODO:
            //  download images
            //  modify src if needs be
            //  return list of images
        }
    }

    let result = [
    ].to_vec();
    result
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
