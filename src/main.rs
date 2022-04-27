use lib_cstars::cache;
use lib_cstars::commands;
use lib_cstars::configuration;
use lib_cstars::errors::Error;
use lib_cstars::http;


mod cli;
mod errors;
use errors::CliError;

fn main() -> Result<(), CliError> {
    let config = configuration::parse_configuration(
        &directories::ProjectDirs::from("", "cstars", "cstars")
            .unwrap()
            .config_dir()
            .join("cstars.toml"),
    )?;
    let client = http::build_client(&config)?;
    let cacher = cache::FileBasedCacher::new(&config);
    let cli = cli::parse_cli_arguments()
        .map_err(|err| err.exit())
        .unwrap();

    let result: Result<String, Error> = match cli.command {
        cli::Commands::Submit { solution, date } => {
            lib_cstars::commands::submit_solution_for_date(cacher, client, date.into(), solution)
        }
        cli::Commands::Get { object } => match object {
            cli::GetType::Input { date } => {
                commands::get_input_for_date(cacher, client, date.into())
            }
            cli::GetType::Description { output, date } => commands::get_description_for_date(
                cacher,
                client,
                date.into(),
                0,
                output.unwrap_or(cli::CliOutputType::Html).into(),
            ),
            cli::GetType::StarCount { date } => {
                commands::get_status_for_date(cacher, client, date.into())
            }
        },
        cli::Commands::Config {} => commands::output_config(&config),
    };
    output_result(result?);
    Ok(())
}

fn output_result(result: String) {
    print!("{}", result);
}

fn output_error(error: Error) {
    eprint!("{}", error);
}
