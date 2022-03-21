use lib_cstars::commands;
use lib_cstars::errors::Error;
use lib_cstars::http;

mod cli;
mod errors;
use errors::CliError;

fn main() -> Result<(), CliError> {
    let client = http::build_client()?;
    let cli = cli::parse_cli_arguments()?;
    let result: Result<String, Error> = match cli.command {
        cli::Commands::Submit { solution, date } => {
            lib_cstars::commands::submit_solution_for_date(client, date.into(), solution)
        }
        cli::Commands::Get { object, date } => match object {
            cli::GetType::Input => commands::get_input_for_date(client, date.into()),
            cli::GetType::Description => commands::get_description_for_date(client, date.into(), 0),
        },
        cli::Commands::Config {} => todo!(),
    };
    Ok(output_result(result?))
}

fn output_result(result: String) {
    print!("{}", result);
}

fn output_error(error: Error) {
    eprint!("{}", error);
}
