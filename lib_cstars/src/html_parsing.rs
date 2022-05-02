use regex;
use std::time;

use crate::errors::{Error, ErrorKind};
use crate::shared::{AnswerStatus, Correctness, Part};
use html_editor as parser;
use parser::operation::{Htmlifiable, Queryable, Selector};

/// Parse the star count for a given day from html
pub fn parse_star_count_from_response(text: String, day: u8) -> Result<u8, Error> {
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
    Ok(0)
}

/// Parse the displayed text on an answer-submission page
pub fn parse_answer_state_from_response_text(response_text: &str) -> Result<AnswerStatus, Error> {
    if response_text.contains("not the right answer") {
        return Ok(AnswerStatus::Correctness(Correctness::Incorrect));
    }
    if response_text.contains("wait") {
        let wait_time: time::Duration = parse_wait_time(response_text);
        return Ok(AnswerStatus::TooRecent(wait_time));
    }
    if response_text.contains("right answer") {
        return Ok(AnswerStatus::Correctness(Correctness::Correct));
    }
    Err(Error::new(ErrorKind::Configuration {
        message: String::from("Failed to parse submission response text"),
    }))
}

fn parse_wait_time(response_text: &str) -> time::Duration {
    // "You have 4m 51s left to wait"
    let regex_string =
        regex::Regex::new(r#"You have ((?P<minutes>\d+)m )??(?P<seconds>\d+)s left to wait"#)
            .unwrap();
    let captures = regex_string.captures(response_text).unwrap();
    let minutes: u64 = match captures.name("minutes") {
        Some(captured) => captured.as_str().parse().unwrap(),
        None => 0,
    };
    let seconds: u64 = captures.name("seconds").unwrap().as_str().parse().unwrap();
    time::Duration::new(seconds + (minutes * 60), 0)
}

/// Convert a list of html elements into markdown
pub fn convert_to_markdown_descriptions(
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

pub fn convert_to_html_descriptions(
    day_descriptions: &[parser::Element],
) -> Result<Vec<String>, Error> {
    let converted_descriptions = day_descriptions.iter().map(|x| x.children.html()).collect();
    Ok(converted_descriptions)
}

pub fn parse_day_description_from_html(response_body: &str) -> Result<Vec<parser::Element>, Error> {
    let html_tree = parser::parse(response_body).map_err(|err| {
        Error::new(ErrorKind::Connection {
            message: format!("Failed to parse response body: [ {} ]", err),
        })
    })?;
    let selector = Selector::from(".day-desc");
    let day_descriptions = html_tree.query_all(&selector);
    Ok(day_descriptions)
}

pub fn select_descriptions_via_part(
    day_descriptions: &[parser::Element],
    part: Part,
) -> Result<&[parser::Element], Error> {
    Ok(match part {
        Part::Both => day_descriptions,
        Part::First => &day_descriptions[0..1],
        Part::Second => &day_descriptions[1..2],
    })
}
