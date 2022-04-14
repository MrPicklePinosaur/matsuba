
mod error;

use std::vec::Vec;
use std::option::Option;

use error::{BoxResult, Error};

// mini argparsing library

type Commands = Vec<Command>;
// type HandlerFn = fn(arg_it: dyn Iterator<Item = String>);
type HandlerFn = fn(flagparse: FlagParse) -> BoxResult<()>;

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
    // pub args: u8, // TODO could make this take named argument names
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
    pub args: Vec<String>,
}

impl Cli {
    
    pub fn run(&self, args: &Vec<String>) -> BoxResult<()> {
        
        let mut arg_it = args.iter();
        arg_it.next(); // skip program name
        let cmd_name = arg_it.next().ok_or(Error::InvalidCommand)?;

        // find command to dispatch
        let cmd: &Command = self.commands
            .iter()
            .find(|c| &c.command_name == cmd_name).ok_or(Error::InvalidCommand)?;

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
                break;
            };

            if flag.is_none() {
                // TODO ugly
                return Err(Box::new(Error::InvalidFlag));
            }
            let flag = flag.unwrap();

            // check if flag is expecting value
            if flag.parameter {
                let value = arg_it.next().ok_or(Error::MissingFlagValue)?;
                flagparse.add_flag_with_value(flag, value);
            } else {
                flagparse.add_flag(flag);
            }

            next = arg_it.next();
        }

        // read rest of arguments
        while next.is_some() {
            flagparse.args.push(next.unwrap().to_string());
            next = arg_it.next();
        }

        // TODO check if all mandatory flags were called

        // pass control to command handler
        let dispatch = cmd.handler;
        dispatch(flagparse)?;

        Ok(())
    }

    fn parse_flags(&self, flagparse: &mut FlagParse) {
        
    }

    pub fn help_message(&self) {

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

    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = desc.to_string();
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
    pub fn long(mut self, long: &str) -> Self {
        self.long = Some(long.to_string());
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

    pub fn get_flag(&self, short: char) -> bool {
        return self.flags.iter().find(|p| p.0.short == short).is_some();
    }

    fn new() -> Self {
        FlagParse {
            flags: Vec::new(),
            args: Vec::new(),
        }
    }

    fn add_flag(&mut self, flag: &'a Flag) {
        self.flags.push((flag, None));
    }

    fn add_flag_with_value(&mut self, flag: &'a Flag, value: &str) {
        self.flags.push((flag, Some(value.to_string())));
    }

}

