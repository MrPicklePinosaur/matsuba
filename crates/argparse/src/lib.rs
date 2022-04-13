
mod error;

use std::vec::Vec;
use std::option::Option;

use error::{BoxResult, Error};

// mini argparsing library

type Commands = Vec<Command>;
// type HandlerFn = fn(arg_it: dyn Iterator<Item = String>);
type HandlerFn = fn(flagparse: FlagParse);

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
    pub required: bool,
    pub parameter: bool,
    pub short: char,
    pub long: Option<String>,
}

pub struct FlagParse<'a> {
    flags: Vec<(&'a Flag, Option<String>)>,
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
        let mut flagparse = FlagParse::new();

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
                return Err(Box::new(Error::new("invalid flag")));
            }
            let flag = flag.unwrap();

            // check if flag is expecting value
            if flag.parameter == false {
                let value = arg_it.next().ok_or(Error::new("expecting value"))?;
                flagparse.add_flag_with_value(flag, value);
            } else {
                flagparse.add_flag(flag);
            }

            next = arg_it.next();
        }

        // TODO check if all mandatory flags were called

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
            required: false,
            parameter: false,
            short: short,
            long: None,
        }
    }

    pub fn desc(mut self, desc: String) -> Self {
        self.desc = desc;
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
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

impl<'a> FlagParse<'a> {

    pub fn get_flag_value(&self, short: char) -> Option<&String> {
        match self.flags.iter().find(|p| p.0.short == short) {
            Some(p) => p.1.as_ref(),
            None => None,
        }
    }

    fn new() -> Self {
        FlagParse {
            flags: Vec::new(),
        }
    }

    fn add_flag(&mut self, flag: &'a Flag) {
        self.flags.push((flag, None));
    }

    fn add_flag_with_value(&mut self, flag: &'a Flag, value: &str) {
        self.flags.push((flag, Some(value.to_string())));
    }

}

