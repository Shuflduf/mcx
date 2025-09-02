use color_eyre::eyre::Result;

mod cli;
mod init;
mod loaders;
mod run;

fn main() -> Result<()> {
    color_eyre::install()?;
    match cli::parse_arguments()? {
        cli::Command::Init => init::setup_server()?,
        cli::Command::Run => run::start_server()?,
    }
    Ok(())
}
