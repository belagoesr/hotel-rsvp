use crate::model::customer::{Customer, CustomerType};
use crate::model::days::Date;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hotels {
    Lakewood,
    Bridgewood,
    Ridgewood,
}

#[derive(Debug, Clone, Copy)]
pub struct WeekdayRate {
    pub regular: i32,
    pub rewards: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct WeekendRate {
    pub regular: i32,
    pub rewards: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Hotel {
    pub hotel_name: Hotels,
    pub weekday_rate: WeekdayRate,
    pub weekend_rate: WeekendRate,
    pub rating: i32,
}

impl Hotel {
    fn get_weekday_rate_for_customer(self, customer: CustomerType) -> i32 {
        match customer {
            CustomerType::Regular => self.weekday_rate.regular,
            CustomerType::Rewards => self.weekday_rate.rewards,
        }
    }
    fn get_weekend_rate_for_customer(self, customer: CustomerType) -> i32 {
        match customer {
            CustomerType::Regular => self.weekend_rate.regular,
            CustomerType::Rewards => self.weekend_rate.rewards,
        }
    }

    pub fn get_rate_for_customer(self, date: &Date, customer: CustomerType) -> i32 {
        if date.is_weekday {
            self.get_weekday_rate_for_customer(customer)
        } else {
            self.get_weekend_rate_for_customer(customer)
        }
    }
}
