use model::Hotels;

pub mod model;
pub mod process_input;

fn get_cheapest_option(input: &str) -> Hotels {
    return Hotels::Bridgewood;
}

#[cfg(test)]
mod tests {
    use crate::{get_cheapest_option, model::Hotels};

    #[test]
    fn check_regular_customer_weekdays() {
        let case = "Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)";
        let output = get_cheapest_option(case);
        assert_eq!(output, Hotels::Lakewood);
    }

    #[test]
    fn check_regular_customer_weekends() {
        let case = "Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)";
        let output = get_cheapest_option(case);
        assert_eq!(output, Hotels::Bridgewood);
    }
}

#[test]
fn check_rewards_customer() {
    let case = "Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)";
    let output = get_cheapest_option(case);
    assert_eq!(output, Hotels::Bridgewood);
}
