use lib_cstars::cache;
use lib_cstars::commands;
use lib_cstars::configuration;
use lib_cstars::errors::Error;
use lib_cstars::http;
use lib_cstars::shared;

mod cli;
mod errors;
use errors::CliError;

fn main() -> Result<(), CliError> {
    let configuration_path = &directories::ProjectDirs::from("", "cstars", "cstars")
        .unwrap()
        .config_dir()
        .join("cstars.toml");

    let config = configuration::parse_configuration(&configuration_path)?;
    let client = http::build_client(&config)?;
    let cacher = cache::FileBasedCacher::new(&config);
    let cli = cli::parse_cli_arguments()
        .map_err(|err| err.exit())
        .unwrap();

    let result: Result<String, Error> = match cli.command {
        cli::Commands::Submit { solution, date } => {
            lib_cstars::commands::submit_solution_for_date(cacher, client, date.into(), &solution)
                .map(|result| match result {
                    shared::AnswerStatus::Repeated => {
                        format!("You repeated a previous answer. It was {solution}")
                    }
                    shared::AnswerStatus::TooRecent => String::from(
                        "You gave your last answer too recently; Wait a couple of seconds",
                    ),
                    shared::AnswerStatus::Correctness(correct) => match correct {
                        shared::Correctness::Incorrect => {
                            format!("Your submitted answer [{solution}] was incorrect")
                        }
                        shared::Correctness::Correct => {
                            format!("Correct answer! Good Job! It was indeed [{solution}]")
                        }
                    },
                })
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
