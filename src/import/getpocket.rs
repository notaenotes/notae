use regex::Regex;
use std::fs;

const FILENAME: &str = "ril_export.html";

#[derive(Debug)]
pub struct NewUrl {
    pub url: String,
    pub tags: Vec<String>,
}

fn read_file() -> String {
    match fs::read_to_string(FILENAME) {
        Ok(text) => text,
        Err(err) => err.to_string(),
    }
}

fn extract_links(file_contents: String) -> Vec<NewUrl> {
    let regular_expression = Regex::new("href=\"(?P<Url>.*?)\".*?tags=\"(?P<Tags>.*?)\"").unwrap();
    let tags_separator: &str = ",";

    regular_expression
        .captures_iter(&file_contents)
        .map(|captured| NewUrl {
            url: String::from(&captured["Url"]),
            tags: captured["Tags"]
                .split(&tags_separator)
                .map(str::to_string)
                .collect(),
        })
        .collect::<Vec<NewUrl>>()
}

pub fn get_links() -> Vec<NewUrl> {
    let file_contents = read_file();
    extract_links(file_contents)
}
