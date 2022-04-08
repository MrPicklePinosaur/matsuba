
use std::vec::Vec;
use std::collections::HashMap;
use std::path::Path;
use roxmltree::{Document, ParsingOptions, Node};

pub struct Conversion {
    r_ele: Vec<String>,
    k_ele: String,
}

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

    // JMdict element node should be the last child
    let root = doc.root().last_child().unwrap();

    for node in root.children().filter(|n| n.is_element()) {
        println!(">>>>> {:?}", node);
        parse_entry(&node);
    }
}

pub fn parse_entry(entry_node: &Node) {

    let mut conversions: HashMap<&str, Conversion> = HashMap::new();

    for elem in entry_node.children().filter(|n| n.is_element()) {
        // println!("{:?}", elem);

        match elem.tag_name().name() {
            "k_ele" => { // parse kanji element 

                // keb guaranteed to exist
                let keb_text = elem
                    .children()
                    .find(|n| n.tag_name().name() == "keb").unwrap()
                    .text().unwrap();

                // ignore duplicate (could also use nightly 'try_insert')
                if conversions.contains_key(keb_text) {
                    return;
                }

                conversions.insert(
                    keb_text,
                    Conversion{ r_ele: Vec::new(), k_ele: keb_text.to_string()}
                );

            },
            "r_ele" => { // parse reading

                // reb guaranteed to exist
                let reb_text = elem
                    .children()
                    .find(|n| n.tag_name().name() == "reb").unwrap()
                    .text().unwrap();

                // check for re_restr (reading only applies to specific kanji elements)
                let mut add_reading_to: Vec<&str> = Vec::new();
                for re_restr_node in elem.children().filter(|n| n.tag_name().name() == "re_restr") {
                    println!("{:?}", re_restr_node);
                    let re_restr_text = re_restr_node.text().unwrap();
                    add_reading_to.push(re_restr_text);
                }

                // if no re_restr, assume all
                if add_reading_to.len() == 0 {
                    for conv in conversions.values_mut() {
                        conv.r_ele.push(reb_text.to_string());
                    }
                } else {
                    for keb in add_reading_to {
                        // TODO maybe check if keb not exist
                        conversions
                            .get_mut(keb).unwrap()
                            .r_ele.push(reb_text.to_string());
                    }
                }

            },
            _ => {},
        }

    }
}
