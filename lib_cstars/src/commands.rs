use crate::shared::Date;
use crate::{errors::Error, http::ADVENT_OF_CODE_URL_BASE};
use html_editor as parser;
use parser::{Htmlifiable, Queryable, Selector};
use reqwest::blocking;

pub fn get_input_for_date(client: blocking::Client, date: Date) -> Result<String, Error> {
    log::trace!("Function: input_for_date called; args:  {:?}", &date);
    let request = client.get(build_input_url(date));
    let response = request.send()?;
    Ok(response.text()?)
}

pub fn submit_solution_for_date(
    client: blocking::Client,
    date: Date,
    solution: String,
) -> Result<String, Error> {
    log::trace!("Function: solution_for_date called; args:  {:?}", date);
    let request = client.post(build_answer_url(date)).body(solution);
    let response = request.send()?;
    Ok(response.text()?)
}

pub fn get_description_for_date(
    client: blocking::Client,
    date: Date,
    part: u8,
) -> Result<String, Error> {
    log::trace!("Function: description_for_date called; args:  {:?}", date);
    let request = client.get(build_date_url(date));
    let response = request.send()?;
    log::debug!(
        "Received response {:?} from [{:?}]",
        response,
        response.url()
    );
    let response_body = response.text()?;
    let response_body = &response_body.replace("\n", "");

    parse_day_description_from_html(response_body, part)
}

fn parse_day_description_from_html(response_body: &str, part: u8) -> Result<String, Error> {
    let html_tree = parser::parse(&response_body).map_err(|err| Error::ConnectionError {
        message: format!("Failed to parse response body: [ {} ]", err),
    })?;
    let selector = Selector::from(".day-desc");
    let day_descriptions: Vec<String> = html_tree
        .query_all(&selector)
        .iter()
        .map(|x| x.children.html())
        .collect();
    Ok(match part {
        0 => day_descriptions.join("\n"),
        1 => day_descriptions[0].to_owned(),
        2 => day_descriptions[1].to_owned(),
        _ => {
            return Err(Error::CommandError {
                message: String::from("Unknown part"),
            })
        }
    })
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
