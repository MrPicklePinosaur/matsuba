
use matsuba::{
    converter,
    x,
    db,
    cli,
    error::BoxResult,
};

fn main() -> BoxResult<()> {

    x::run_x()?;
    // cli::runcli()?;
    Ok(())

}

