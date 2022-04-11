
use std::vec::Vec;
use std::collections::HashMap;
use std::path::Path;
use roxmltree::{Document, ParsingOptions, Node};

use super::db::{Connection, Entry};
use super::db::insert_entry;
use super::error::{BoxResult, SimpleError};

pub fn parse_jmdict_xml(conn: &Connection, path: &Path) -> BoxResult<()> {

    let text = std::fs::read_to_string(path).unwrap();
    let opt = ParsingOptions { allow_dtd: true };
    let doc = Document::parse_with_options(&text, opt)?;

    // JMdict element node should be the last child
    let root = doc.root().last_child().unwrap();

    for node in root.children().filter(|n| n.is_element()) {
        println!(">>>>> {:?}", node);
        let entries = parse_entry(&node)?;

        // insert into db (not sure if this should be done here)
        for (k, v) in entries.iter() {
            println!("{}", k);
            println!("{:?}", v.r_ele);
            insert_entry(&conn, v)?;
        }
    }

    Ok(())
}

pub fn parse_entry(entry_node: &Node) -> BoxResult<HashMap<String, Entry>> {

    let mut entries: HashMap<String, Entry> = HashMap::new();

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
                if entries.contains_key(keb_text) {
                    // return Err(SimpleError::new(""));
                    break;
                }

                entries.insert(
                    keb_text.to_string(),
                    Entry{ r_ele: Vec::new(), k_ele: keb_text.to_string()}
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
                    let re_restr_text = re_restr_node.text().unwrap();
                    add_reading_to.push(re_restr_text);
                }

                // if no re_restr, assume all
                if add_reading_to.len() == 0 {
                    for conv in entries.values_mut() {
                        conv.r_ele.push(reb_text.to_string());
                    }
                } else {
                    for keb in add_reading_to {
                        entries
                            .get_mut(keb).ok_or(SimpleError::new("keb does not exist"))?
                            .r_ele.push(reb_text.to_string());
                    }
                }

            },
            _ => {},
        }

    }

    Ok(entries)
}
