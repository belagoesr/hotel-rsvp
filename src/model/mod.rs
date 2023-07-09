pub mod customer;
pub mod days;
pub mod hotels;

use crate::model::customer::Customer;
use crate::model::days::Date;

#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    pub customer: Customer,
    pub date_range: Vec<Date>,
}
