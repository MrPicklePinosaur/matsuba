use roxmltree::{Document, Node, ParsingOptions};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::vec::Vec;

use log::debug;

use super::db::insert_entry;
use super::db::{DBConnection, Entry};

use crate::error::BoxResult;

#[derive(Debug)]
pub enum XmlError {
    Fetch(String),
    KebNotExist,
}
impl Error for XmlError {}
impl Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch(e) => write!(f, "fetch error: {:?}", e),
            Self::KebNotExist => write!(f, "keb does not exist"),
        }
    }
}

/// Downloads jmdict
pub async fn fetch_jmdict_xml(tempfile_path: &Path, target_path: &Path) -> Result<(), XmlError> {
    use std::fs::File;

    const DICT_URL: &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz";

    let body = reqwest::get(DICT_URL)
        .await
        .map_err(|e| XmlError::Fetch(e.to_string()))?
        .bytes()
        .await
        .map_err(|e| XmlError::Fetch(e.to_string()))?;

    let mut file = File::create(tempfile_path).map_err(|e| XmlError::Fetch(e.to_string()))?;
    file.write_all(body.as_ref())
        .map_err(|e| XmlError::Fetch(e.to_string()))?;

    // TODO use library to do this instead
    let decoded = Command::new("gzip")
        .args(vec!["-dc", tempfile_path.to_str().unwrap()])
        .output()
        .map_err(|e| XmlError::Fetch(e.to_string()))?;

    let mut file = File::create(target_path).map_err(|e| XmlError::Fetch(e.to_string()))?;
    file.write_all(&decoded.stdout).unwrap();

    Ok(())
}

pub fn parse_jmdict_xml(
    conn: &mut DBConnection,
    path: &Path,
    tags: &HashSet<&str>,
) -> BoxResult<()> {
    let text = std::fs::read_to_string(path).unwrap();
    let opt = ParsingOptions { allow_dtd: true };
    let doc = Document::parse_with_options(&text, opt)?;

    // JMdict element node should be the last child
    let root = doc.root().last_child().unwrap();

    let tx = conn.transaction()?;
    for node in root.children().filter(|n| n.is_element()) {
        parse_entry(&tx, &node, tags)?;
    }

    tx.commit()?;
    Ok(())
}

fn parse_entry(conn: &DBConnection, entry_node: &Node, tags: &HashSet<&str>) -> BoxResult<()> {
    let mut entries: HashMap<String, Vec<Entry>> = HashMap::new();

    for elem in entry_node.children().filter(|n| n.is_element()) {
        match elem.tag_name().name() {
            "dial" | "field" | "ke_inf" | "misc" | "pos" | "re_inf" => {
                // check if we are accepting the tag
                let tag = elem.text().unwrap();
                if tags.get(tag).is_none() {
                    continue;
                }
            }
            "k_ele" => {
                // parse kanji element

                // keb guaranteed to exist
                let keb_text = elem
                    .children()
                    .find(|n| n.tag_name().name() == "keb")
                    .unwrap()
                    .text()
                    .unwrap();

                // ignore duplicate (could also use nightly 'try_insert')
                if entries.contains_key(keb_text) {
                    // return Err(SimpleError::new(""));
                    break;
                }

                entries.insert(keb_text.to_string(), Vec::new());
            }
            "r_ele" => {
                // parse reading

                // reb guaranteed to exist
                let reb_text = elem
                    .children()
                    .find(|n| n.tag_name().name() == "reb")
                    .unwrap()
                    .text()
                    .unwrap();

                // check for re_restr (reading only applies to specific kanji elements)
                let mut add_reading_to: Vec<&str> = Vec::new();
                for re_restr_node in elem
                    .children()
                    .filter(|n| n.tag_name().name() == "re_restr")
                {
                    let re_restr_text = re_restr_node.text().unwrap();
                    add_reading_to.push(re_restr_text);
                }

                // if no re_restr, assume all
                if add_reading_to.is_empty() {
                    for (keb, conv) in entries.iter_mut() {
                        conv.push(Entry::new(reb_text.to_string(), keb.to_string()));
                    }
                } else {
                    for keb in add_reading_to {
                        entries
                            .get_mut(keb)
                            .ok_or(XmlError::KebNotExist)?
                            .push(Entry::new(reb_text.to_string(), keb.to_string()));
                    }
                }
            }
            _ => {}
        }
    }

    for group in entries.values() {
        for entry in group.iter() {
            debug!("{} - {}", entry.k_ele, entry.r_ele);
            insert_entry(conn, entry)?;
        }
    }
    Ok(())
}
