use std::collections::HashMap;
use std::collections::LinkedList;

use crate::conversion::*;

// TODO ownership in this entire module is fucked, please fix sometime

type StateHandle = usize;
const START_STATE: StateHandle = 0;

#[derive(Debug)]
pub struct State {
    pub accepting: Option<(String, String)>,
    pub transitions: HashMap<char, StateHandle>,
    pub depth: usize, // distance from staring state
}

#[derive(Debug)]
pub struct Converter {
    pub state_pool: Vec<State>,
    pub state_handle: StateHandle,
    pub output: String,
    pub input: LinkedList<char>,
}

impl State {
    pub fn new(depth: usize) -> Self {
        State {
            accepting: None, // empty string does not accept
            transitions: HashMap::new(),
            depth,
        }
    }
}

impl Converter {
    pub fn new() -> Converter {
        Converter {
            state_pool: build_dfa(),
            state_handle: START_STATE,
            output: String::from(""), // stack structure
            input: LinkedList::new(), // queue structure
        }
    }

    pub fn input_char(&mut self, ch: char) {
        self.input.push_front(ch);
        self.step_dfa();
    }

    pub fn del_char(&mut self) {
        self.output.pop();
        self.state_handle = START_STATE;
    }

    pub fn accept(&mut self) -> String {
        self.state_handle = START_STATE;
        self.input.clear();

        let out = self.output.clone();
        self.output.clear();
        out
    }

    fn step_dfa(&mut self) {
        if self.input.is_empty() {
            return; // maybe output a warning
        }

        let ch = self.input.pop_back().unwrap();
        let lowercase_ch = ch.to_ascii_lowercase();
        let prev_ch = self.output.chars().last();

        // attempt to transition on input character
        match self.state_pool[self.state_handle]
            .transitions
            .get(&lowercase_ch)
        {
            Some(x) => {
                self.state_handle = *x;
            }
            None => {
                self.state_handle = START_STATE;

                // attempt transition again but from start_state
                match self.state_pool[self.state_handle]
                    .transitions
                    .get(&lowercase_ch)
                {
                    Some(x) => {
                        self.state_handle = *x;
                    }
                    None => {}
                }
            }
        };

        // small tsu expansion
        if prev_ch.is_some() {
            let prev_ch = prev_ch.unwrap();
            if ch == prev_ch && REPEATABLE_CHARACTERS.contains(&lowercase_ch) {
                self.output.pop();

                let small_tsu = *match ch.is_ascii_lowercase() {
                    true => HIRAGANA_SMALL_TSU,
                    false => KATAKANA_SMALL_TSU,
                };
                self.output.push(small_tsu);
            }
        }

        self.output.push(ch);

        // check if we are in accepting state
        match self.state_pool[self.state_handle].accepting {
            Some(ref x) => {
                let mut is_lower: bool = false;
                for _ in 0..self.state_pool[self.state_handle].depth {
                    is_lower = self.output.pop().unwrap().is_ascii_lowercase();
                }

                // decide if converting hiragana or katakana
                let output_ch = match is_lower {
                    true => &x.0,
                    false => &x.1,
                };
                self.output.push_str(output_ch);
                self.state_handle = START_STATE;
            }
            None => {}
        }
    }
}

// TODO this function is sorta hard to read (maybe refactor)
pub fn build_dfa() -> Vec<State> {
    let mut state_pool: Vec<State> = vec![State::new(0)];

    for conv in CONVERSION_TABLE {
        let mut cur_state: StateHandle = START_STATE;

        for (i, ch) in conv.0.chars().enumerate() {
            // create state of does not exist
            if !state_pool
                .get(cur_state)
                .unwrap()
                .transitions
                .contains_key(&ch)
            {
                state_pool.push(State::new(i + 1));
                let new_state_handle = state_pool.len() - 1;
                state_pool
                    .get_mut(cur_state)
                    .unwrap()
                    .transitions
                    .insert(ch, new_state_handle);
            }

            // transition
            cur_state = *state_pool
                .get(cur_state)
                .unwrap()
                .transitions
                .get(&ch)
                .unwrap();

            // mark as accepting if last char
            if i == conv.0.len() - 1 {
                state_pool.get_mut(cur_state).unwrap().accepting =
                    Some((conv.1.to_string(), conv.2.to_string()));
            }
        }
    }

    state_pool
}
