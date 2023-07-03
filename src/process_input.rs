// TODO: There must be a way to supply the application with the input data via text file.
use crate::model::*;

pub fn process_line(s: &str) -> ParsedInput {
    let splitted: Vec<&str> = s.split(':').collect();

    let mut regular_customer = true;
    if splitted[0].contains("Regular") {
        regular_customer = true;
    }
    if splitted[0].contains("Rewards") {
        regular_customer = false;
    }

    let customer = Customer {
        customer_type: if regular_customer {
            CustomerType::Regular
        } else {
            CustomerType::Rewards
        },
    };

    let dates = splitted[1].split(',').map(|s| {
        let start_bytes = s.find('(').unwrap_or(0);
        let end_bytes = s.find(')').unwrap_or(s.len());
        let day = &s[start_bytes + 1..end_bytes];

        Date {
            day: 1,
            month: "jan".to_string(),
            year: 2000,
            is_weekday: match day {
                "sat" => false,
                "sun" => false,
                _ => true,
            },
        }
    });
    ParsedInput {
        customer,
        date_range: dates.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::process_line;
    use crate::model::{Customer, CustomerType, Hotels};

    #[test]
    fn process_line_weekdays_customer_regular() {
        let case = "Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)";
        let output = process_line(case);
        assert_eq!(output.customer.customer_type, CustomerType::Regular);
        assert_eq!(output.date_range.len(), 3);
        assert_eq!(output.date_range[0].is_weekday, true);
        assert_eq!(output.date_range[1].is_weekday, true);
        assert_eq!(output.date_range[2].is_weekday, true);
    }
    #[test]
    fn process_line_weekends_customer_regular() {
        let case = "Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)";
        let output = process_line(case);
        assert_eq!(output.customer.customer_type, CustomerType::Regular);
        assert_eq!(output.date_range.len(), 3);
        assert_eq!(output.date_range[0].is_weekday, true);
        assert_eq!(output.date_range[1].is_weekday, false);
        assert_eq!(output.date_range[2].is_weekday, false);
    }

    #[test]
    fn process_line_weekends_customer_rewards() {
        let case = "Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)";
        let output = process_line(case);
        assert_eq!(output.customer.customer_type, CustomerType::Rewards);
        assert_eq!(output.date_range.len(), 3);
        assert_eq!(output.date_range[0].is_weekday, true);
        assert_eq!(output.date_range[1].is_weekday, true);
        assert_eq!(output.date_range[2].is_weekday, false);
    }
}
