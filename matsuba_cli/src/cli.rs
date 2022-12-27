use log::{debug, info};
use matsuba_grpc::matsuba_client::MatsubaClient;
use matsuba_grpc::{ConvertRequest, FetchRequest, GetStateRequest, SetStateHenkanRequest};
use pino_argparse::{Cli, Command, Flag, FlagParse};
use tonic::Request;

use tokio::runtime::Runtime;

use std::error::Error;
use std::fmt;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum CliError {
    WrongArgCount,
    InvalidTag(String),
}

impl Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::WrongArgCount => write!(f, "Wrong number of arguments"),
            CliError::InvalidTag(tag) => write!(f, "Invalid tag passed: {}", tag),
        }
    }
}

static HELP_MSG: &str = "\
USAGE:
matsucli [-v] <command>

COMMANDS:
run
fetch <word-lists>
state <query>
convert <phrase>
";
// state is for getting info about the daemon
// like current kana mode etc (useful for scripts)

static CONNECTION_STRING: &str = "http://[::1]:10000";

pub fn runcli() -> BoxResult<()> {
    let cli = Cli {
        program_name: "matsucli",
        synopsis: "simple japanese ime",
        subcommands: vec![
            Command {
                command_name: "run",
                desc: "run matsuba daemon",
                handler: handle_run,
                flags: vec![],
            },
            Command {
                command_name: "unlock",
                desc: "removes lock in the event of a crash",
                handler: handle_unlock,
                flags: vec![],
            },
            Command {
                command_name: "fetch",
                desc: "fetch word lists",
                handler: handle_fetch,
                flags: vec![Flag::new("tags")
                    .short('t')
                    .desc("specify which tags should be included or not included")
                    .parameter()],
            },
            Command {
                command_name: "convert",
                desc: "use the matsuba input converter",
                handler: handle_convert,
                flags: vec![
                    Flag::new("kana")
                        .short('k')
                        .desc("only perform kana conversion"),
                    Flag::new("count")
                        .short('c')
                        .desc("limit for number of conversions to output")
                        .parameter(),
                ],
            },
            Command {
                command_name: "state",
                desc: "query state of matsuba",
                handler: handle_state,
                flags: vec![],
            },
        ],
        global_flags: vec![],
        ..Default::default()
    };

    let args = std::env::args().collect();
    cli.run(&args)?;

    Ok(())
}

fn handle_run(_flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

fn handle_unlock(_flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

fn handle_fetch(flagparse: FlagParse) -> BoxResult<()> {
    if flagparse.args.is_empty() {
        return Err(Box::new(CliError::WrongArgCount));
    }

    // tag customization
    let mut default_tags = matsuba_common::all_tags();
    let tag_options = flagparse
        .get_flag_value::<String>("tags")
        .unwrap_or_default();
    for option in tag_options.split(',') {
        let (mode, tag) = option.split_at(1);
        if tag.is_empty() {
            return Err(Box::new(CliError::InvalidTag(tag.to_owned())));
        }

        if mode == "+" {
            default_tags.insert(tag);
        } else if mode == "-" {
            default_tags.remove(tag);
        } else {
            return Err(Box::new(CliError::InvalidTag(tag.to_owned())));
        }
    }

    // TODO this may be pretty inefficient
    let tags = default_tags
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    Runtime::new()?.block_on(async {
        let mut client = MatsubaClient::connect(CONNECTION_STRING).await.unwrap();

        let _response = client
            .fetch(Request::new(FetchRequest {
                tags,
                database_path: flagparse.args[0].clone(),
            }))
            .await
            .unwrap();
    });
    Ok(())
}

fn handle_convert(flagparse: FlagParse) -> BoxResult<()> {
    Runtime::new()?.block_on(async {
        // TODO proper error handling inside async block
        let mut client = MatsubaClient::connect(CONNECTION_STRING).await.unwrap();

        let response = client
            .convert(Request::new(ConvertRequest {
                // TODO only taking first arg for now
                raw: flagparse.args.get(0).unwrap().to_string(),
                kana_only: flagparse.get_flag("kana"),
                result_count: flagparse.get_flag_value::<usize>("count").unwrap_or(1) as i32,
            }))
            .await
            .unwrap();
        debug!("{:?}", response);
    });
    Ok(())
}

fn handle_state(flagparse: FlagParse) -> BoxResult<()> {
    enum Mode {
        Get,
        Henkan,
        Muhenkan,
    }

    let mode = if flagparse.args.len() == 0 {
        Mode::Get
    } else {
        match flagparse.args.get(0).unwrap().as_str() {
            "henkan" => Mode::Henkan,
            "muhenkan" => Mode::Muhenkan,
            "get" => Mode::Get,
            _ => {
                // TODO proper error handle
                return Ok(());
            }
        }
    };

    Runtime::new()?.block_on(async {
        let mut client = MatsubaClient::connect(CONNECTION_STRING).await.unwrap();

        match mode {
            Mode::Get => {
                let response = client
                    .get_state(Request::new(GetStateRequest {}))
                    .await
                    .unwrap();

                debug!("{:?}", response);
            }
            Mode::Henkan => {
                let response = client
                    .set_state_henkan(Request::new(SetStateHenkanRequest {}))
                    .await
                    .unwrap();

                debug!("{:?}", response);
            }
            _ => {}
        }
    });

    Ok(())
}
