
use std::collections::HashMap;

static CONVERSION_TABLE: &'static [(&str, &str)] = &[
    ("a",  "あ"),
    ("i",  "い"),
    ("u",  "う"),
    ("e",  "え"),
    ("o",  "お"),
    ("ka", "か"),
    ("ki", "き"),
    ("ku", "く"),
    ("ke", "け"),
    ("ko", "こ"),
];

#[derive(Debug)]
pub struct State<'a> {
    pub accepting: Option<&'a str>,
    pub transitions: HashMap<&'a str, Box<State<'a>>>
}

#[derive(Debug)]
pub struct Converter<'a> {
    pub dfa: State<'a>
}

impl Converter<'_> {

    pub fn new() -> Self {
        let converter = Converter{
            dfa: State{
                accepting: None, // empty string does not accept
                transitions: HashMap::new(),
            }
        };
        converter.build_dfa();
        converter
    }

    fn build_dfa(&self) {

    }

}

