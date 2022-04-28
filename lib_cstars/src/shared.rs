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
    GetDescription,
    GetStars,
    PostAnswer,
}

impl core::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title: &str = match self {
            RequestType::GetInput => "input",
            RequestType::GetDescription => "descriptions",
            RequestType::GetStars => "stars",
            RequestType::PostAnswer => "answers",
        };
        write!(f, "{title}")
    }
}

pub fn specify_request(date: &Date, request_type: RequestType) -> RequestSpecification {
    return RequestSpecification {
        date: date.clone(),
        request_type,
    };
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
