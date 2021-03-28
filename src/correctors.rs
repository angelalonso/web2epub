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
    let result = [
        "images/default-vpc-diagram.png",
        "images/nondefault-vpc-diagram.png",
    ].to_vec();
    result
}

