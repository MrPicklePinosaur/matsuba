
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
    pub transitions: HashMap<char, Box<State<'a>>>
}

#[derive(Debug)]
pub struct Converter<'a> {
    pub dfa: Box<State<'a>>
}

impl State<'_> {

    pub fn new() -> Self {
        State{
            accepting: None, // empty string does not accept
            transitions: HashMap::new(),
        }
    }
}

impl Converter<'_> {

    pub fn new() -> Self {
        let converter = Converter{
            dfa: Box::new(State::new()),
        };
        converter.build_dfa()
    }

    fn build_dfa(mut self) -> Self {
        
        for conv in CONVERSION_TABLE {
            let mut cur_state: &mut Box<State> = &mut self.dfa;
            for (i, ch) in conv.0.chars().enumerate() {

                // create state of does not exist
                if (!cur_state.transitions.contains_key(&ch)) {
                    let new_state = State::new();
                    cur_state.transitions.insert(ch, Box::new(new_state));
                }

                // transition
                cur_state = cur_state.transitions.get_mut(&ch).unwrap();

                // mark as accepting if last char
                if (i == conv.0.len()-1) {
                    cur_state.accepting = Some(conv.1);
                } 
            }
        }
        self
    }

}

