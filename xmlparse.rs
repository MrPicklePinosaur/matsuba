
use std::path::Path;
use roxmltree::{Document, ParsingOptions};

pub fn parse_jmdict_xml(path: &Path) {

    let text = std::fs::read_to_string(path).unwrap();
    let opt = ParsingOptions { allow_dtd: true };
    let doc = match Document::parse_with_options(&text, opt) {
        Ok(doc) => doc,
        Err(e) => {
            println!("error: {}", e);
            return;
        },
    };

    let it = doc.descendants().filter(|n| !n.is_comment());

}

pub fn parse_entry() {

}
