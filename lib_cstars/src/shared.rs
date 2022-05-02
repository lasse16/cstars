#[derive(Debug, Clone)]
pub struct Date {
    pub day: u8,
    pub year: u16,
}

pub enum OutputFormat {
    Html,
    Markdown,
}

pub struct RequestSpecification {
    pub date: Date,
    pub request_type: RequestType,
}

pub enum RequestType {
    GetInput,
    GetDescription(Part),
    GetStars,
    PostAnswer,
}

impl core::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title: &str = match self {
            RequestType::GetInput => "input",
            RequestType::GetDescription(part) => match part {
                Part::Both => "descriptions_both",
                Part::First => "descriptions_first",
                Part::Second => "descriptions_second",
            },
            RequestType::GetStars => "stars",
            RequestType::PostAnswer => "answers",
        };
        write!(f, "{title}")
    }
}

pub fn specify_request(date: &Date, request_type: RequestType) -> RequestSpecification {
    RequestSpecification {
        date: date.clone(),
        request_type,
    }
}

pub enum AnswerStatus {
    Repeated,
    TooRecent(std::time::Duration),
    Correctness(Correctness),
}

pub enum Correctness {
    Incorrect,
    Correct,
}

pub enum Part {
    Both,
    First,
    Second,
}
impl From<u8> for Part {
    fn from(part: u8) -> Self {
        match part {
            0 => Part::Both,
            1 => Part::First,
            2 => Part::Second,
            _ => panic!(),
        }
    }
}
