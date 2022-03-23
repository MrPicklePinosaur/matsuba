
use std::collections::HashMap;

use super::conversion::{CONVERSION_TABLE};

#[derive(Debug)]
pub struct State {
    pub accepting: Option<String>,
    pub transitions: HashMap<char, Box<State>>
}

#[derive(Debug)]
pub struct Converter<'a> {
    pub start_state: &'a State,
    pub cur_state: &'a State,
    pub output: String,
    pub depth: u8, // depth of state in dfa (root is 0)
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
        Converter{
            start_state: start_state,
            cur_state: start_state,
            output: String::from(""),
            depth: 0,
        }
    }

    pub fn consume_char(&mut self, ch: char) {

        self.output.push(ch);
        self.cur_state = match self.cur_state.transitions.get(&ch) {
            Some(ref x) => {
                self.depth += 1;
                x
            },
            None => {
                self.depth = 0;
                self.start_state
            }
        };

        // check if we are in accepting state
        match self.cur_state.accepting {
            Some(ref x) => {
                for i in 0..self.depth {
                    self.output.pop();
                }
                self.output.push_str(x);
                self.cur_state = self.start_state;
                self.depth = 0;
            },
            None => {}
        }
    }
}

pub fn build_dfa() -> State {
    
    let mut new_dfa = State::new();

    for conv in CONVERSION_TABLE {
        let mut cur_state: &mut State = &mut new_dfa;
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

