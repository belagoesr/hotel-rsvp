use std::fs::read_to_string;

use model::{Hotel, Hotels, ParsedInput, WeekdayRate, WeekendRate};
use process_input::process_line;

pub mod model;
pub mod process_input;

pub fn get_hotel_data(hotel: Hotels) -> Hotel {
    match hotel {
        Hotels::Lakewood => Hotel {
            hotel_name: Hotels::Lakewood,
            weekday_rate: WeekdayRate {
                regular: 110,
                rewards: 80,
            },
            weekend_rate: WeekendRate {
                regular: 90,
                rewards: 80,
            },
            rating: 3,
        },
        Hotels::Bridgewood => Hotel {
            hotel_name: Hotels::Bridgewood,
            weekday_rate: WeekdayRate {
                regular: 160,
                rewards: 110,
            },
            weekend_rate: WeekendRate {
                regular: 60,
                rewards: 50,
            },
            rating: 4,
        },
        Hotels::Ridgewood => Hotel {
            hotel_name: Hotels::Ridgewood,
            weekday_rate: WeekdayRate {
                regular: 220,
                rewards: 100,
            },
            weekend_rate: WeekendRate {
                regular: 150,
                rewards: 40,
            },
            rating: 5,
        },
    }
}

pub fn get_available_hotels() -> Vec<Hotel> {
    [Hotels::Lakewood, Hotels::Bridgewood, Hotels::Ridgewood]
        .iter()
        .map(|hotel| get_hotel_data(*hotel))
        .collect()
}
pub fn get_quote_for_hotel(hotel: Hotel, input: &ParsedInput) -> i32 {
    input.date_range.iter().fold(0, |acc, x| {
        acc + hotel.get_rate_for_customer(x, input.customer.customer_type)
    })
}

pub fn get_cheapest_option(input: ParsedInput) -> Hotel {
    get_available_hotels()
        .into_iter()
        .map(|hotel| (hotel, get_quote_for_hotel(hotel, &input)))
        .reduce(|acc, pair| match acc.1 == pair.1 {
            true => {
                if acc.0.rating > pair.0.rating {
                    acc
                } else {
                    pair
                }
            }
            false => {
                if acc.1 < pair.1 {
                    acc
                } else {
                    pair
                }
            }
        })
        .map(|(hotel, _)| hotel)
        .unwrap()
}

pub fn get_hotels_for_input() -> Vec<Hotels> {
    let filename = "input.txt";
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let output = process_line(line);
        let cheapest_hotel = get_cheapest_option(output);
        result.push(cheapest_hotel.hotel_name);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_hotels_for_input, model::Hotels};

    #[test]
    fn check_get_hotels_for_input() {
        let result = get_hotels_for_input();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Hotels::Lakewood);
        assert_eq!(result[1], Hotels::Bridgewood);
        assert_eq!(result[2], Hotels::Ridgewood);
    }
}
