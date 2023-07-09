use std::fs::read_to_string;

use crate::io::process_input::process_line;
use crate::model::{
    hotels::{Hotel, Hotels, WeekdayRate, WeekendRate},
    ParsedInput,
};

fn get_hotel_data(hotel: Hotels) -> Hotel {
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

fn get_quote_for_hotel(hotel: Hotel, input: &ParsedInput) -> i32 {
    input.date_range.iter().fold(0, |acc, x| {
        acc + hotel.get_rate_for_customer(x, input.customer.customer_type)
    })
}

fn get_cheapest_option(input: ParsedInput) -> Hotel {
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

pub fn get_available_hotels() -> Vec<Hotel> {
    [Hotels::Lakewood, Hotels::Bridgewood, Hotels::Ridgewood]
        .iter()
        .map(|hotel| get_hotel_data(*hotel))
        .collect()
}

pub async fn get_cheapest_hotels_for_input() -> Vec<Hotels> {
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
    use crate::{
        logic::{
            get_available_hotels, get_cheapest_hotels_for_input, get_hotel_data,
            get_quote_for_hotel,
        },
        model::{
            customer::{Customer, CustomerType},
            days::{Date, Day, Month},
            hotels::Hotels,
            ParsedInput,
        },
    };

    #[tokio::test]
    async fn check_get_cheapest_hotels_for_input() {
        let result = get_cheapest_hotels_for_input().await;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Hotels::Lakewood);
        assert_eq!(result[1], Hotels::Bridgewood);
        assert_eq!(result[2], Hotels::Ridgewood);
    }

    #[test]
    fn check_get_available_hotels() {
        let hotels = get_available_hotels();
        assert_eq!(hotels.len(), 3);
        assert_eq!(hotels[0].hotel_name, Hotels::Lakewood);
        assert_eq!(hotels[1].hotel_name, Hotels::Bridgewood);
        assert_eq!(hotels[2].hotel_name, Hotels::Ridgewood);
    }

    #[test]
    fn check_get_lakewood_hotel_data() {
        let lakewood = get_hotel_data(Hotels::Lakewood);
        assert_eq!(lakewood.hotel_name, Hotels::Lakewood);
        assert_eq!(lakewood.rating, 3);
        assert_eq!(lakewood.weekday_rate.regular, 110);
        assert_eq!(lakewood.weekday_rate.rewards, 80);
        assert_eq!(lakewood.weekend_rate.regular, 90);
        assert_eq!(lakewood.weekend_rate.rewards, 80);
    }

    #[test]
    fn check_get_bridgewood_hotel_data() {
        let bridgewood = get_hotel_data(Hotels::Bridgewood);
        assert_eq!(bridgewood.hotel_name, Hotels::Bridgewood);
        assert_eq!(bridgewood.rating, 4);
        assert_eq!(bridgewood.weekday_rate.regular, 160);
        assert_eq!(bridgewood.weekday_rate.rewards, 110);
        assert_eq!(bridgewood.weekend_rate.regular, 60);
        assert_eq!(bridgewood.weekend_rate.rewards, 50);
    }

    #[test]
    fn check_get_ridgewood_hotel_data() {
        let ridgewood = get_hotel_data(Hotels::Ridgewood);
        assert_eq!(ridgewood.hotel_name, Hotels::Ridgewood);
        assert_eq!(ridgewood.rating, 5);
        assert_eq!(ridgewood.weekday_rate.regular, 220);
        assert_eq!(ridgewood.weekday_rate.rewards, 100);
        assert_eq!(ridgewood.weekend_rate.regular, 150);
        assert_eq!(ridgewood.weekend_rate.rewards, 40);
    }

    #[test]
    fn check_get_quote_for_hotel() {
        let parsed_input = ParsedInput {
            customer: Customer {
                customer_type: CustomerType::Regular,
            },
            date_range: vec![
                Date {
                    day: Day::Friday,
                    month: Month::January,
                    year: 2023,
                    is_weekday: true,
                },
                Date {
                    day: Day::Saturday,
                    month: Month::January,
                    year: 2023,
                    is_weekday: false,
                },
            ],
        };
        let hotel = get_hotel_data(Hotels::Lakewood);
        let quote = get_quote_for_hotel(hotel, &parsed_input);
        assert_eq!(quote, 200i32);
    }
}
