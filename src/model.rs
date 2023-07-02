use chrono::Datelike;

#[derive(Debug, PartialEq)]
pub enum Hotels {
    Lakewood,
    Bridgewood,
    Ridgewood,
}

#[derive(Debug, PartialEq)]
pub enum CustomerType {
    Regular,
    Rewards,
}

#[derive(Debug)]
pub struct Hotel {
    pub regular_rate: f32,
    pub weekend_rate: f32,
    pub rewards_customer_rate: f32,
    pub rating: i32,
}

impl Hotel {
    fn get_rate_for_date(self, input_date: Date) -> f32 {
        if input_date.is_weekday {
            self.regular_rate
        } else {
            self.weekend_rate
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Customer {
    pub customer_type: CustomerType,
}

#[derive(Debug, PartialEq)]
pub struct Date {
    // TODO: swap for a specific datetime type?
    pub day: i32,
    pub month: String,
    pub year: i32,
    pub is_weekday: bool,
}
#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    pub customer: Customer,
    pub date_range: Vec<Date>,
}
