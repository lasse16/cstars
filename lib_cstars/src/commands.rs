use crate::shared::{specify_request, Date, OutputFormat, RequestType};
use crate::{
    cache::Cacher,
    configuration::Configuration,
    errors::{Error, ErrorKind},
    http::ADVENT_OF_CODE_URL_BASE,
};
use html_editor as parser;
use parser::operation::{Htmlifiable, Queryable, Selector};
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
    let result = parse_answer_state_from_response_text(&response_text)?;
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

    let star_count: u8 = parse_star_count_from_response(response.text()?, date.day)?;
    let result = star_count.to_string();
    cacher.overwrite(&request_spec, &result);
    Ok(result)
}

fn parse_star_count_from_response(text: String, day: u8) -> Result<u8, Error> {
    let html_tree = parser::parse(&text).map_err(|err| {
        Error::new(ErrorKind::Connection {
            message: format!("Failed to parse response body: [ {} ]", err),
        })
    })?;
    let selector = Selector::from(format!(".calendar-day{}", day).as_str());
    let day_element = html_tree.query(&selector).unwrap();
    let (_, class_attribute) = day_element
        .attrs
        .iter()
        .find(|(name, _)| name == "class")
        .unwrap();
    if class_attribute.contains("calendar-complete") {
        return Ok(1);
    }
    if class_attribute.contains("calendar-verycomplete") {
        return Ok(2);
    }
    return Ok(0);
}

fn parse_answer_state_from_response_text(response_text: &str) -> Result<AnswerStatus, Error> {
    if response_text.contains("not the right answer") {
        return Ok(AnswerStatus::Correctness(Correctness::Incorrect));
    }
    if response_text.contains("wait") {
        return Ok(AnswerStatus::TooRecent);
    }
    if response_text.contains("right answer") {
        return Ok(AnswerStatus::Correctness(Correctness::Correct));
    }
    Err(Error::new(ErrorKind::Configuration {
        message: String::from("Failed to parse submission response text"),
    }))
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

    let day_descriptions = parse_day_description_from_html(&response_body)?;
    let selected_day_descriptions = select_descriptions_via_part(&day_descriptions, part)?;
    let converted_descriptions = match output_format {
        OutputFormat::Html => convert_to_html_descriptions(selected_day_descriptions),
        OutputFormat::Markdown => convert_to_markdown_descriptions(selected_day_descriptions),
    }?;
    let result = converted_descriptions.join("\n");
    cacher.overwrite(&request_spec, &result);
    return Ok(result);
}

fn convert_to_markdown_descriptions(
    selected_day_descriptions: &[parser::Element],
) -> Result<Vec<String>, Error> {
    let mut output = Vec::<String>::new();
    for day_desc in selected_day_descriptions {
        let mut converted_day_desc = Vec::<String>::with_capacity(day_desc.children.len());
        // We can safely ignore the day_desc element as it is only a marker
        for child in &day_desc.children {
            converted_day_desc.push(convert_node_to_markdown(child));
        }
        output.push(converted_day_desc.join("\n"));
    }
    Ok(output)
}

fn convert_node_to_markdown(node: &parser::Node) -> String {
    match node {
        parser::Node::Element { .. } => convert_html_tag_to_markdown(&node.clone().into_element()),
        parser::Node::Text(text) => text.to_string(),
        parser::Node::Comment(_) => "".to_string(),
        parser::Node::Doctype(_) => "".to_string(),
    }
}

fn convert_html_tag_to_markdown(element: &parser::Element) -> String {
    let (prefix, postfix) = match &*element.name {
        "p" => ("\n", ""),
        // Necessary whitespace in prefix
        "h1" => ("# ", ""),
        "h2" => ("## ", ""),
        "em" | "i" => ("*", "*"),
        "code" => ("```", "```"),
        "pre" => ("", ""),
        "a" => ("", ""),
        "span" => ("", ""),
        _ => ("unknown tag", ""),
    };

    let mut contained_information: Vec<String> = Vec::new();
    for child in &element.children {
        contained_information.push(convert_node_to_markdown(child));
    }

    format!("{}{}{}", prefix, &contained_information.join(""), postfix)
}

fn select_descriptions_via_part(
    day_descriptions: &[parser::Element],
    part: u8,
) -> Result<&[parser::Element], Error> {
    Ok(match part {
        0 => day_descriptions,
        1 => &day_descriptions[0..1],
        2 => &day_descriptions[1..2],
        _ => {
            return Err(Error::new(ErrorKind::Command {
                message: format!("Requested an unknown part [ {:?} ]", part),
            }))
        }
    })
}

fn convert_to_html_descriptions(
    day_descriptions: &[parser::Element],
) -> Result<Vec<String>, Error> {
    let converted_descriptions = day_descriptions.iter().map(|x| x.children.html()).collect();
    Ok(converted_descriptions)
}

fn parse_day_description_from_html(response_body: &str) -> Result<Vec<parser::Element>, Error> {
    let html_tree = parser::parse(&response_body).map_err(|err| {
        Error::new(ErrorKind::Connection {
            message: format!("Failed to parse response body: [ {} ]", err),
        })
    })?;
    let selector = Selector::from(".day-desc");
    let day_descriptions = html_tree.query_all(&selector);
    return Ok(day_descriptions);
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

pub enum AnswerStatus {
    Repeated,
    TooRecent,
    Correctness(Correctness),
}
pub enum Correctness {
    Incorrect,
    Correct,
}
