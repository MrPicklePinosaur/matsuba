
use argparse::{Cli, Command, Flag, FlagParse};

use super::error::{BoxResult};
use super::db;
use super::converter::{Converter, build_dfa};

static HELP_MSG: &str = "\
USAGE:
matsuba [-v] <command>

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
        program_name: "matsuba".to_string(),
        synopsis: String::new(),
        commands: vec![
            Command {
                desc: "run matsuba daemon".to_string(),
                command_name: "run".to_string(),
                handler: handle_run,
                flags: vec![],
            },
            Command {
                desc: "fetch word lists".to_string(),
                command_name: "fetch".to_string(),
                handler: handle_fetch,
                flags: vec![],
            },
            Command {
                desc: "use the matsuba input converter".to_string(),
                command_name: "convert".to_string(),
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
        ],
        global_flags: vec![],
    };

    let args = std::env::args().collect();
    cli.run(&args)?;

    Ok(())
}

fn handle_run(flagparse: FlagParse) -> BoxResult<()> {
    println!("run command");

    Ok(())
}

fn handle_fetch(flagparse: FlagParse) -> BoxResult<()> {

    let path = std::path::Path::new("./tests/jmdict_full.xml");
    let mut conn = db::get_connection()?;

    // db::init(&conn)?;
    // xmlparse::parse_jmdict_xml(&mut conn, path)?;

    Ok(())
}

fn handle_convert(flagparse: FlagParse) -> BoxResult<()> {

    let dfa = build_dfa();
    let mut c = Converter::new(&dfa);

    let conn = db::get_connection()?;

    for input in flagparse.args.iter() {

        for ch in input.chars() {
            c.input_char(ch);
        }
        let kana = c.accept();

        // if kana flag is passed, don't do any more conversion
        if flagparse.get_flag('k') {
            println!("{}", kana);
            continue;
        }

        let count = flagparse.get_flag_value::<usize>('c').unwrap_or(1);

        let converted = db::search(&conn, &kana)?;
        for c in converted.iter().take(count) {
            println!("{}", c.k_ele);
        }
    }

    Ok(())
}

