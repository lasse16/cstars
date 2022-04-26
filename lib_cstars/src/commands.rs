use crate::shared::{Date, OutputFormat};
use crate::{errors::Error, http::ADVENT_OF_CODE_URL_BASE};
use html_editor as parser;
use parser::prelude::{Htmlifiable, Queryable};
use parser::Selector;
use reqwest::blocking;

pub fn get_input_for_date(client: blocking::Client, date: Date) -> Result<String, Error> {
    log::trace!("Function: input_for_date called; args:  {:?}", &date);
    let request = client.get(build_input_url(&date));
    let response = request.send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::CommandError {
            message: format!("Requested input for missing date [ {:?} ]", &date),
        });
    }
    Ok(response.text()?)
}

enum Correctness {
    IncorrectAnswer,
    TooRecentAnswer,
    CorrectAnswer,
}

pub fn submit_solution_for_date(
    client: blocking::Client,
    date: Date,
    solution: String,
) -> Result<String, Error> {
    log::trace!("Function: solution_for_date called; args:  {:?}", date);
    let form_params =
        std::collections::hash_map::HashMap::from([("answer", solution.as_str()), ("level", "1")]);
    let request = client.post(build_answer_url(&date)).form(&form_params);
    let response = request.send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::CommandError {
            message: format!("Submitted solution for missing date [ {:?} ]", &date),
        });
    }
    let response_text = response.text()?;
    Ok(
        match parse_solution_correctness_from_response(&response_text)? {
            Correctness::IncorrectAnswer => {
                format!("Your answer was incorrect, answer [ {} ]", &solution)
            }
            Correctness::TooRecentAnswer => {
                format!("Your last answer was given too recent")
            }
            Correctness::CorrectAnswer => {
                format!(
                    "Your answer was correct. Good job! answer [ {} ]",
                    &solution
                )
            }
        },
    )
}

pub fn get_status_for_date(client: blocking::Client, date: Date) -> Result<String, Error> {
    let request = client.get(build_year_url(&date));
    let response = request.send()?;

    let star_count: u8 = parse_star_count_from_response(response.text()?, date.day)?;
    Ok(star_count.to_string())
}

pub fn parse_star_count_from_response(text: String, day: u8) -> Result<u8, Error> {
    let html_tree = parser::parse(&text).map_err(|err| Error::ConnectionError {
        message: format!("Failed to parse response body: [ {} ]", err),
    })?;
    //FIXME It's a workaround due to https://github.com/lomirus/html_editor/issues/4
    let selector = Selector::from(format!(".calendar-day{}", day).as_str());
    let selector_one_star =
        Selector::from(format!(".calendar-day{} calendar-complete", day).as_str());
    let selector_two_stars =
        Selector::from(format!(".calendar-day{} calendar-verycomplete", day).as_str());
    if html_tree.query(&selector).is_some() {
        return Ok(0);
    }
    if html_tree.query(&selector_one_star).is_some() {
        return Ok(1);
    }
    if html_tree.query(&selector_two_stars).is_some() {
        return Ok(2);
    } else {
        return Err(Error::ConnectionError {
            message: "Failed to parse star count from css classes".to_string(),
        });
    }
}

fn parse_solution_correctness_from_response(response_text: &str) -> Result<Correctness, Error> {
    if response_text.contains("not the right answer") {
        return Ok(Correctness::IncorrectAnswer);
    }
    if response_text.contains("wait") {
        return Ok(Correctness::TooRecentAnswer);
    }
    if response_text.contains("right answer") {
        return Ok(Correctness::CorrectAnswer);
    }
    Err(Error::ConfigurationError {
        message: String::from("Failed to parse submission response text"),
    })
}

pub fn get_description_for_date(
    client: blocking::Client,
    date: Date,
    part: u8,
    output_format: OutputFormat,
) -> Result<String, Error> {
    log::trace!("Function: description_for_date called; args:  {:?}", date);
    let request = client.get(build_day_url(&date));
    let response = request.send()?;
    log::debug!(
        "Received response {:?} from [{:?}]",
        response,
        response.url()
    );
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::CommandError {
            message: format!("Requested description for missing date [ {:?} ]", &date),
        });
    }
    let response_body = response.text()?;

    let day_descriptions = parse_day_description_from_html(&response_body)?;
    let selected_day_descriptions = select_descriptions_via_part(&day_descriptions, part)?;
    let converted_descriptions = match output_format {
        OutputFormat::Html => convert_to_html_descriptions(selected_day_descriptions),
        OutputFormat::Markdown => convert_to_markdown_descriptions(selected_day_descriptions),
    }?;
    return Ok(converted_descriptions.join("\n"));
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
        parser::Node::Doctype => "".to_string(),
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
            return Err(Error::CommandError {
                message: format!("Requested an unknown part [ {:?} ]", part),
            })
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
    let html_tree = parser::parse(&response_body).map_err(|err| Error::ConnectionError {
        message: format!("Failed to parse response body: [ {} ]", err),
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

pub fn output_config(config: &crate::configuration::Configuration) -> Result<String, Error> {
    Ok(format!("{:?}", config))
}
