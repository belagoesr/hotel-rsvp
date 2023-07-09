pub mod days;
pub mod hotels;

use crate::model::days::Date;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CustomerType {
    Regular,
    Rewards,
}

#[derive(Debug, PartialEq)]
pub struct Customer {
    pub customer_type: CustomerType,
}

#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    pub customer: Customer,
    pub date_range: Vec<Date>,
}
