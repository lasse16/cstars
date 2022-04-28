use crate::shared::Date;

pub const ADVENT_OF_CODE_URL_BASE: &str = "https://adventofcode.com";
pub fn build_input_url(date: &Date) -> String {
    return format!("{}/input", build_day_url(date));
}

pub fn build_answer_url(date: &Date) -> String {
    return format!("{}/answer", build_day_url(date));
}

pub fn build_day_url(date: &Date) -> String {
    return format!("{}/day/{}", build_year_url(date), date.day);
}

pub fn build_year_url(date: &Date) -> String {
    return format!("{ADVENT_OF_CODE_URL_BASE}/{}", date.year);
}
