use crate::shared::{specify_request, AnswerStatus, Date, OutputFormat, RequestType};
use crate::{
    cache::Cacher,
    configuration::Configuration,
    errors::{CommandErrorKind, Error, ErrorKind},
    html_parsing, url,
};
use reqwest::blocking;

pub fn get_input_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
) -> Result<String, Error> {
    let request_spec = specify_request(&date, RequestType::GetInput);

    if let Some(cached_result) = cacher.lookup(&request_spec) {
        return Ok(cached_result);
    }

    let response = request_from_url(client, url::build_input_url(&date))?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            kind: CommandErrorKind::MissingDate(date),
        }));
    }
    let result = response.text()?;

    cacher.overwrite(&request_spec, &result);

    Ok(result)
}

fn request_from_url(client: blocking::Client, url: String) -> Result<blocking::Response, Error> {
    let request = client.get(url);
    let response = request.send()?;
    Ok(response)
}

pub fn submit_solution_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
    solution: &String,
) -> Result<AnswerStatus, Error> {
    let request_spec = specify_request(&date, RequestType::PostAnswer);
    if let Some(cached_result) = cacher.lookup(&request_spec) {
        let mut cached_previous_answer_attempts = cached_result.lines();
        if cached_previous_answer_attempts
            .find(|&attempt| attempt == solution)
            .is_some()
        {
            return Ok(AnswerStatus::Repeated);
        }
    }

    let form_params =
        std::collections::hash_map::HashMap::from([("answer", solution.as_str()), ("level", "1")]);
    let request = client.post(url::build_answer_url(&date)).form(&form_params);
    let response = request.send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            kind: CommandErrorKind::MissingDate(date),
        }));
    }
    let response_text = response.text()?;
    let result = html_parsing::parse_answer_state_from_response_text(&response_text)?;
    if let AnswerStatus::Correctness(_) = result {
        cacher.append(&request_spec, solution);
    }
    Ok(result)
}

pub fn get_status_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
) -> Result<String, Error> {
    let request_spec = specify_request(&date, RequestType::GetStars);
    if let Some(cached_result) = cacher.lookup(&request_spec) {
        return Ok(cached_result);
    }

    let response = request_from_url(client, url::build_year_url(&date))?;

    let star_count: u8 = html_parsing::parse_star_count_from_response(response.text()?, date.day)?;
    let result = star_count.to_string();
    cacher.overwrite(&request_spec, &result);
    Ok(result)
}

pub fn get_description_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
    part: u8,
    output_format: OutputFormat,
) -> Result<String, Error> {
    let request_spec = specify_request(&date, RequestType::GetDescription);
    if let Some(cached_result) = cacher.lookup(&request_spec) {
        return Ok(cached_result);
    }

    let response = request_from_url(client, url::build_day_url(&date))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            kind: CommandErrorKind::MissingDate(date),
        }));
    }
    let response_body = response.text()?;

    let day_descriptions = html_parsing::parse_day_description_from_html(&response_body)?;
    let selected_day_descriptions =
        html_parsing::select_descriptions_via_part(&day_descriptions, part)?;
    let converted_descriptions = match output_format {
        OutputFormat::Html => html_parsing::convert_to_html_descriptions(selected_day_descriptions),
        OutputFormat::Markdown => {
            html_parsing::convert_to_markdown_descriptions(selected_day_descriptions)
        }
    }?;
    let result = converted_descriptions.join("\n");
    cacher.overwrite(&request_spec, &result);
    Ok(result)
}

pub fn output_config(config: &Configuration) -> Result<String, Error> {
    Ok(format!("{:?}", config))
}
