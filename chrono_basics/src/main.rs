use chrono_basics::{
    calculate_duration_between_two_dates, format_date_time, parse_date_from_a_string,
};

fn main() {
    let date_str = "2023-12-19";

    println!(
        "Parsed date: {}",
        parse_date_from_a_string(date_str).unwrap()
    );

    let date1 = chrono::NaiveDate::from_ymd_opt(2023, 12, 9).unwrap();
    let date2 = chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

    println!(
        "Duration between dates: {} days",
        calculate_duration_between_two_dates(date1, date2)
    );

    let utc_now = chrono::Utc::now();

    println!("Formatted DateTime: {}", format_date_time(utc_now));
}
