mod commands;
mod http;
mod shared;
use shared::Date;

fn main() -> Result<(), reqwest::Error> {
    let client = http::build_client()?;
    let result: String =
        commands::get_description_for_date(client, Date { day: 8, year: 2020 }, 1)?;
    output_result(result);
    Ok(())
}

fn output_result(result: String) {
    print!("{}", result);
}
