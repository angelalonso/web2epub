#[allow(dead_code)]
pub fn do_correct_images() -> &'static str {
    "test"
}

pub fn get_images(content: String) -> Vec<&'static str> {
    //TODO:
    //  find images
    //  download images
    //  modify src if needs be
    //  return list of images
    get_from_string(content, "<img", ">");
    let result = [
        "images/default-vpc-diagram.png",
        "images/nondefault-vpc-diagram.png",
    ].to_vec();
    result
}

/// Gets the exact parts of a String fitting a pattern
/// First use case: find an image definition in HTML
pub fn get_from_string(content: String, search_start: &str, search_end: &str) {
    let starts: Vec<_> = content.match_indices(search_start).collect();
    let ends: Vec<_> = content.match_indices(search_end).collect();
    let mut pos = [].to_vec();
    for s in starts {
        //let p: Vec<usize>;
        //p.append(&mut s.0);
        let mut p = [s.0].to_vec();
        let mut e_found = false;
        let mut ix = 0;
        while !e_found && ix < ends.len() {
            if ends[ix].0 > s.0 {
                e_found = true;
                p.append(&mut [ends[ix].0].to_vec());
            }
            ix += 1;
        }
        pos.append(&mut [p].to_vec());
        //println!("{:?}", pos);
    }
    for st in pos {
        println!("{}", &content[st[0]..st[1]+1]);
    }
    
}
