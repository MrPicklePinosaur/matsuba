
use std::collections::HashSet;

pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}
use tonic::Request;
use matsubaproto::matsuba_client::MatsubaClient;
use matsubaproto::{ConvertRequest, ConvertResponse};
use argparse::{Cli, Command, Flag, FlagParse};

use matsuba::error::{BoxResult, SimpleError};

use tokio::runtime::Runtime;

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

pub fn runcli() -> BoxResult<()> {

    let cli = Cli {
        program_name: "matsucli",
        synopsis: "simple japanese ime",
        commands: vec![
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
                flags: vec![
                    Flag::new('t')
                        .long("tags")
                        .desc("specify which tags should be included or not included")
                        .parameter(),
                ],
            },
            Command {
                command_name: "convert",
                desc: "use the matsuba input converter",
                handler: handle_convert,
                flags: vec![
                    Flag::new('k')
                        .long("kana")
                        .desc("only perform kana conversion"),
                    Flag::new('c')
                        .long("count")
                        .desc("limit for number of conversions to output")
                        .parameter(),
                ],
            },
            Command {
                command_name: "state",
                desc: "query and mutate state of matsuba",
                handler: handle_state,
                flags: vec![
                    Flag::new('h')
                        .long("henkan")
                        .desc("enable conversion"),
                    Flag::new('H')
                        .long("muhenkan")
                        .desc("disable conversion"),
                ],
            }
        ],
        global_flags: vec![],
    };

    let args = std::env::args().collect();
    cli.run(&args)?;

    Ok(())
}

fn handle_run(flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

fn handle_unlock(flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

fn handle_fetch(flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

fn handle_convert(flagparse: FlagParse) -> BoxResult<()> {
    
    Runtime::new()?.block_on(async {

        // TODO proper error handling inside async block
        let mut client = MatsubaClient::connect("http://[::1]:10000").await.unwrap();

        let response = client.convert(Request::new(
            ConvertRequest {
                // TODO only taking first arg for now
                raw: flagparse.args.get(0).unwrap().to_string(),
                kana_only: flagparse.get_flag('k'),
                result_count: flagparse.get_flag_value::<usize>('c').unwrap_or(1) as i32,
            }
        )).await.unwrap();
        println!("{:?}", response);

    });
    Ok(())
}

fn handle_state(flagparse: FlagParse) -> BoxResult<()> {
    todo!()
}

