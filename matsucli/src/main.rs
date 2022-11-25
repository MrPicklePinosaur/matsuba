mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().format_timestamp(None).init();

    cli::runcli()?;

    Ok(())
}
