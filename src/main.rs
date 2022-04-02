use lib_cstars::commands;
use lib_cstars::configuration;
use lib_cstars::errors::Error;
use lib_cstars::http;
use std::path::Path;

mod cli;
mod errors;
use errors::CliError;

fn main() -> Result<(), CliError> {
    let config =
        configuration::parse_configuration(Path::new(r#"/home/lasse/.config/cstars/cstars.toml"#))?;
    let client = http::build_client(&config)?;
    let cli = match cli::parse_cli_arguments() {
        Ok(cli) => cli,
        // Do not wrap clap error as their error reporting is too nice
        Err(err) => err.exit(),
    };

    let result: Result<String, Error> = match cli.command {
        cli::Commands::Submit { solution, date } => {
            lib_cstars::commands::submit_solution_for_date(client, date.into(), solution)
        }
        cli::Commands::Get { object, date } => match object {
            cli::GetType::Input => commands::get_input_for_date(client, date.into()),
            cli::GetType::Description => commands::get_description_for_date(client, date.into(), 0),
        },
        cli::Commands::Config {} => commands::output_config(&config),
    };
    Ok(output_result(result?))
}

fn output_result(result: String) {
    print!("{}", result);
}

fn output_error(error: Error) {
    eprint!("{}", error);
}
