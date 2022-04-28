use crate::shared::{specify_request, AnswerStatus, Date, OutputFormat, RequestType};
use crate::{
    cache::Cacher,
    configuration::Configuration,
    errors::{Error, ErrorKind},
    html_parsing,
    http::ADVENT_OF_CODE_URL_BASE,
};
use reqwest::blocking;

pub fn get_input_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
) -> Result<String, Error> {
    log::trace!("Function: input_for_date called; args:  {:?}", &date);

    let request_spec = specify_request(&date, RequestType::GetInput);
    if let Some(cached_result) = cacher.lookup(&request_spec) {
        return Ok(cached_result);
    }

    let request = client.get(build_input_url(&date));
    let response = request.send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            message: format!("Requested input for missing date [ {:?} ]", &date),
        }));
    }
    let result = response.text()?;

    cacher.overwrite(&request_spec, &result);

    Ok(result)
}

pub fn submit_solution_for_date<T: Cacher<String>>(
    cacher: T,
    client: blocking::Client,
    date: Date,
    solution: &String,
) -> Result<AnswerStatus, Error> {
    log::trace!("Function: solution_for_date called; args:  {:?}", date);

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
    let request = client.post(build_answer_url(&date)).form(&form_params);
    let response = request.send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            message: format!("Submitted solution for missing date [ {:?} ]", &date),
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

    let request = client.get(build_year_url(&date));
    let response = request.send()?;

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
    log::trace!("Function: description_for_date called; args:  {:?}", date);
    let request_spec = specify_request(&date, RequestType::GetDescription);
    if let Some(cached_result) = cacher.lookup(&request_spec) {
        return Ok(cached_result);
    }

    let request = client.get(build_day_url(&date));
    let response = request.send()?;
    log::debug!(
        "Received response {:?} from [{:?}]",
        response,
        response.url()
    );
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::new(ErrorKind::Command {
            message: format!("Requested description for missing date [ {:?} ]", &date),
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
    return Ok(result);
}

fn build_input_url(date: &Date) -> String {
    return format!("{}/input", build_day_url(date));
}

fn build_answer_url(date: &Date) -> String {
    return format!("{}/answer", build_day_url(date));
}

fn build_day_url(date: &Date) -> String {
    return format!("{}/day/{}", build_year_url(date), date.day);
}

fn build_year_url(date: &Date) -> String {
    return format!("{ADVENT_OF_CODE_URL_BASE}/{}", date.year);
}

pub fn output_config(config: &Configuration) -> Result<String, Error> {
    Ok(format!("{:?}", config))
}
