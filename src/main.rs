
use matsuba::{
    converter,
    x,
    db,
    cli,
    error::BoxResult,
};

fn main() -> BoxResult<()> {

    cli::runcli()?;
    Ok(())

}

