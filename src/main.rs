use lib_cstars::commands;
use lib_cstars::errors::{ConnectionError };
use lib_cstars::http;

mod cli;

fn main() {
    let client = http::build_client().unwrap();
    let cli = cli::parse_cli_arguments();
    let result: Result<String, ConnectionError> = match cli.command {
        cli::Commands::Submit { solution, date } => {
            lib_cstars::commands::submit_solution_for_date(client, date.into(), solution)
        }
        cli::Commands::Get { object, date } => match object {
            cli::GetType::Input => commands::get_input_for_date(client, date.into()),
            cli::GetType::Description => commands::get_description_for_date(client, date.into(), 0),
        },
        cli::Commands::Config {} => todo!(),
    };
    output_result(result.unwrap());
}

fn output_result(result: String) {
    print!("{}", result);
}
