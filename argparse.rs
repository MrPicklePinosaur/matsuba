
use std::vec::Vec;
use std::option::Option;

use super::error::{BoxResult, SimpleError};

// mini argparsing library

type Commands = Vec<Command>;
// type HandlerFn = fn(arg_it: dyn Iterator<Item = String>);
type HandlerFn = fn();

pub struct Cli {
    pub program_name: String,
    pub synopsis: String,
    pub commands: Vec<Command>,
    pub global_flags: Vec<Flag>,
}

pub struct Command {
    pub desc: String,
    pub command_name: String,
    pub handler: HandlerFn,
    pub flags: Vec<Flag>,
}

pub struct Flag {
    pub desc: String,
    pub optional: bool,
    pub parameter: bool,
    pub short: char,
    pub long: Option<String>,
}

impl Cli {
    
    pub fn run(self, args: &Vec<String>) -> BoxResult<()> {
        
        let mut arg_it = args.iter();
        arg_it.next(); // skip program name
        let cmd_name = arg_it.next().unwrap();

        // find command to dispatch
        let cmd: &Command = self.commands
            .iter()
            .find(|c| &c.command_name == cmd_name).unwrap();

        // parse flags for command
        let mut next = arg_it.next();
        while next.is_some() {

            let cur_arg = next.unwrap();

            let flag: Option<&Flag> = if cur_arg.starts_with("--") {
                // TODO maybe unneeded copy
                cmd.flags.iter().find(|f| f.long == Some(cur_arg[2..].to_string()))
            } else if cur_arg.starts_with("-") {
                cmd.flags.iter().find(|f| Some(f.short) == cur_arg.chars().nth(1))
            } else {
                None
            };

            if flag.is_none() {
                // TODO ugly
                return Err(Box::new(SimpleError::new("invalid flag")));
            }


            next = arg_it.next();
        }

        // pass control to command handler
        let dispatch = cmd.handler;
        // dispatch(arg_it);

        Ok(())
    }
}

impl Flag {

    pub fn new(short: char) -> Self {
        Flag {
            desc: String::new(),
            optional: true,
            parameter: false,
            short: short,
            long: None,
        }
    }

    pub fn desc(mut self, desc: String) -> Self {
        self.desc = desc;
        self
    }
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    pub fn parameter(mut self) -> Self {
        self.parameter = true;
        self
    }
    pub fn long(mut self, long: String) -> Self {
        self.long = Some(long);
        self
    }

}

