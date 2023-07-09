#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    // TODO: swap for a specific datetime type?
    pub day: Day,
    pub month: Month,
    pub year: i32,
    pub is_weekday: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
