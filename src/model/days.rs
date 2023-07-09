#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    // TODO: swap for a specific datetime type?
    pub day: i32,
    pub month: String,
    pub year: i32,
    pub is_weekday: bool,
}
