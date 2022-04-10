
use std::vec::Vec;
use std::option::Option;

// mini argparsing library

type Commands = Vec<Command>;
type HandlerFn = fn(arg_it: dyn Iterator<Item = String>);

pub struct Cli {
    program_name: String,
    synopsis: String,
    commands: Vec<Command>,
    global_flags: Vec<Flag>,
}

pub struct Command {
    desc: String,
    command_name: String,
    handler: HandlerFn,
    flags: Vec<Flag>,
}

pub struct Flag {
    desc: String,
    optional: bool,
    short: String,
    long: Option<String>,
}

impl Cli {
    
    pub fn run(self, args: &Vec<String>) {
        
        let mut arg_it = args.iter();
        _ = arg_it.next(); // skip program name
        let cmd_name = arg_it.next().unwrap();

        // find command to dispatch
        let cmd: &Command = self.commands
            .iter()
            .find(|c| &c.command_name == cmd_name).unwrap();

        // pass control to command handler
        let dispatch = cmd.handler;
        // dispatch(arg_it);

    }

}
