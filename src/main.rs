mod commands;
mod http;
mod shared;
use shared::Date;

const ADVENT_OF_CODE_URL_BASE: &str = "https://adventofcode.com";
fn main() -> Result<(), reqwest::Error> {
    let client = http::build_client()?;
    let result: String = commands::get_input_for_date(client, Date { day: 8, year: 2020 })?;
    output_result(result);
    Ok(())
}

fn output_result(result: String) {
    print!("{}", result);
}
