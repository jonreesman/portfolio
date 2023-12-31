use std::io::Read;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;


fn read_file(filename: String) -> String {
    let assets_path = std::env::current_dir().unwrap();
    let mut file = File::open(format!("{}/assets/markdown/{}", assets_path.to_str().unwrap(), filename)).unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    buffer
}

fn clean_latex(file: String) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((.+)\)\^(.+)").unwrap());
    let cleaned_file = file.lines().map(|line| {
        let mut new_line = line.to_owned();
        if line.contains("$$") {
            if line.contains("*") {
                let first_dollar_sign = line.find("$").unwrap_or(line.len());
                let updated_line: String = line.split("").enumerate().map(|(idx, char)| {
                    if idx < first_dollar_sign {
                        return char;
                    }
                    if char == "*" {
                        return " \\cdot ";
                    }
                    return char;
                }).collect::<Vec<&str>>().join("");
                
                new_line = updated_line;
            }
            if RE.is_match(&new_line) {
                new_line = new_line.replace("(", " \\left( ");
                new_line = new_line.replace(")", " \\right) ");
            }
        }
        new_line
    }).collect::<Vec<String>>().join("\n");

    cleaned_file
}

pub fn get_markdown(filepath: String) -> String {
    let file = read_file(filepath);
    // Latex *'s dont jive well with pulldown_cmark since it
    // just assumes our converted mathml is markdown, making
    // our *'s just italics operators
    let cleaned_file = clean_latex(file);
    let mathml = latex2mathml::replace(&cleaned_file).unwrap();
    let parser = pulldown_cmark::Parser::new(&mathml);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
