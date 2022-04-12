#[derive(Debug)]
pub struct Date {
    pub day: u8,
    pub year: u16,
}

pub enum OutputFormat {
    Html,
    Markdown,
}
