use crate::errors::{CommandErrorKind, Error, ErrorKind};
use crate::shared::{AnswerStatus, Correctness};
use html_editor as parser;
use parser::operation::{Htmlifiable, Queryable, Selector};

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

pub fn parse_answer_state_from_response_text(response_text: &str) -> Result<AnswerStatus, Error> {
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
pub fn convert_node_to_markdown(node: &parser::Node) -> String {
    match node {
        parser::Node::Element { .. } => convert_html_tag_to_markdown(&node.clone().into_element()),
        parser::Node::Text(text) => text.to_string(),
        parser::Node::Comment(_) => "".to_string(),
        parser::Node::Doctype(_) => "".to_string(),
    }
}

pub fn convert_html_tag_to_markdown(element: &parser::Element) -> String {
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
    part: u8,
) -> Result<&[parser::Element], Error> {
    Ok(match part {
        0 => day_descriptions,
        1 => &day_descriptions[0..1],
        2 => &day_descriptions[1..2],
        _ => {
            return Err(Error::new(ErrorKind::Command {
                kind: CommandErrorKind::UnknownPart(part),
            }))
        }
    })
}
