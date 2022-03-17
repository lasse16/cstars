use crate::shared::Date;
use crate::ADVENT_OF_CODE_URL_BASE;
use reqwest::blocking;

pub fn get_input_for_date(
    client: blocking::Client,
    date: Date,
) -> Result<String, reqwest::Error> {
    let request = client.get(build_input_url(date));
    let response = request.send()?;
    Ok(response.text()?)
}

pub fn submit_solution_for_date(
    client: blocking::Client,
    date: Date,
    solution: String,
) -> Result<String, reqwest::Error> {
    let request = client.post(build_answer_url(date)).body(solution);
    let response = request.send()?;
    Ok(response.text()?)
}
fn build_input_url(date: Date) -> String {
    return format!("{}/input", build_date_url(date));
}

fn build_answer_url(date: Date) -> String {
    return format!("{}/answer", build_date_url(date));
}

fn build_date_url(date: Date) -> String {
    return format!("{ADVENT_OF_CODE_URL_BASE}/{}/day/{}", date.year, date.day);
}
