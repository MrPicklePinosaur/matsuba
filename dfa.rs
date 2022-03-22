
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
pub struct State {
    pub accepting: Option<String>,
    pub transitions: HashMap<char, Box<State>>
}

#[derive(Debug)]
pub struct Converter<'a> {
    pub start_state: &'a State,
    pub cur_state: &'a State,
}

impl State {

    pub fn new() -> Self {
        State{
            accepting: None, // empty string does not accept
            transitions: HashMap::new(),
        }
    }
}

impl<'a> Converter<'a> {

    pub fn new(start_state: &'a State) -> Converter<'a> {
        let converter = Converter{
            start_state: start_state,
            cur_state: start_state,
        };
        converter
    }

    fn consume_char(mut self, ch: char) {

    }

}

fn build_dfa() -> State {
    
    let mut new_dfa = State::new();

    let mut cur_state: &mut State = &mut new_dfa;
    for conv in CONVERSION_TABLE {
        for (i, ch) in conv.0.chars().enumerate() {

            // create state of does not exist
            if !cur_state.transitions.contains_key(&ch) {
                let new_state = State::new();
                cur_state.transitions.insert(ch, Box::new(new_state));
            }

            // transition
            cur_state = cur_state.transitions.get_mut(&ch).unwrap();

            // mark as accepting if last char
            if i == conv.0.len()-1 {
                cur_state.accepting = Some(conv.1.to_string());
            } 
        }
    }

    new_dfa
}

