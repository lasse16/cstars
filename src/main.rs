use lib_cstars::commands;
use lib_cstars::http;
use lib_cstars::shared;
use lib_cstars::shared::Date;

fn main() {
    let client = http::build_client().unwrap();
    let result: String =
        commands::get_description_for_date(client, Date { day: 8, year: 2020 }, 1).unwrap();
    output_result(result);
}

fn output_result(result: String) {
    print!("{}", result);
}
