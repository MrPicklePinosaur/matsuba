
use std::collections::HashMap;
use std::collections::LinkedList;

use super::conversion::*;

#[derive(Debug)]
pub struct State {
    pub accepting: Option<(String, String)>,
    pub transitions: HashMap<char, Box<State>>,
    pub depth: usize, // distance from staring state
}

#[derive(Debug)]
pub struct Converter<'a> {
    pub start_state: &'a State,
    pub cur_state: &'a State,
    pub output: String,
    pub input: LinkedList<char>,
}

impl State {

    pub fn new(depth: usize) -> Self {
        State{
            accepting: None, // empty string does not accept
            transitions: HashMap::new(),
            depth: depth,
        }
    }
}

impl<'a> Converter<'a> {

    pub fn new(start_state: &'a State) -> Converter<'a> {
        Converter{
            start_state: start_state,
            cur_state: start_state,
            output: String::from(""),  // stack structure
            input: LinkedList::new(),  // queue structure
        }
    }

    pub fn input_char(&mut self, ch: char) {
        self.input.push_front(ch);
        self.step_dfa();
    }

    pub fn del_char(&mut self) {
        self.output.pop();
        self.cur_state = self.start_state;
    }

    pub fn accept(&mut self) -> String {
        self.cur_state = self.start_state;
        self.input.clear();

        let out = self.output.clone();
        self.output.clear();
        return out;
    }

    fn step_dfa(&mut self) {

        if self.input.is_empty() {
            return; // maybe output a warning
        }

        let ch = self.input.pop_back().unwrap();
        let lowercase_ch = ch.to_ascii_lowercase();
        let prev_ch = self.output.chars().last();

        // attempt to transition on input character
        match self.cur_state.transitions.get(&lowercase_ch) {
            Some(ref x) => {
                self.cur_state = x;
            },
            None => {
                self.cur_state = self.start_state;

                // attempt transition again but from start_state
                match self.cur_state.transitions.get(&lowercase_ch) {
                    Some(ref x) => { self.cur_state = x; },
                    None => {},
                }
            },
        };

        // small tsu expansion
        if prev_ch.is_some() {
            let prev_ch = prev_ch.unwrap().clone();
            if ch == prev_ch && REPEATABLE_CHARACTERS.contains(&lowercase_ch) {
                self.output.pop();

                let small_tsu = match ch.is_ascii_lowercase() {
                    true  => HIRAGANA_SMALL_TSU,
                    false => KATAKANA_SMALL_TSU,
                }.clone();
                self.output.push(small_tsu);
            }
        }

        self.output.push(ch);

        // check if we are in accepting state
        match self.cur_state.accepting {
            Some(ref x) => {

                let mut is_lower: bool = false;
                for _ in 0..self.cur_state.depth {
                    is_lower = self.output.pop().unwrap().is_ascii_lowercase();
                }

                // decide if converting hiragana or katakana
                let output_ch = match is_lower {
                    true => &x.0,
                    false => &x.1,
                };
                self.output.push_str(output_ch);
                self.cur_state = self.start_state;
            },
            None => {}
        }

    }
}

pub fn build_dfa() -> State {
    
    let mut new_dfa = State::new(0);

    for conv in CONVERSION_TABLE {
        let mut cur_state: &mut State = &mut new_dfa;
        for (i, ch) in conv.0.chars().enumerate() {

            // create state of does not exist
            if !cur_state.transitions.contains_key(&ch) {
                let new_state = State::new(i+1);
                cur_state.transitions.insert(ch, Box::new(new_state));
            }

            // transition
            cur_state = cur_state.transitions.get_mut(&ch).unwrap();

            // mark as accepting if last char
            if i == conv.0.len()-1 {
                cur_state.accepting = Some((conv.1.to_string(), conv.2.to_string()));
            } 
        }
    }
    new_dfa
}

