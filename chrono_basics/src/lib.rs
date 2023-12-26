use chrono::prelude::*;

pub fn parse_date_from_a_string(date_str: &str) -> Result<DateTime<Utc>, &'static str> {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(parsed_date) => {
            let naive_time = match NaiveTime::from_hms_opt(0, 0, 0) {
                Some(time) => time,
                None => {
                    return Err("Invalid time");
                }
            };

            let naive_datetime = parsed_date.and_time(naive_time);

            let datetime = Utc.from_utc_datetime(&naive_datetime);
            Ok(datetime)
        }
        Err(_) => Err("Error parsing date"),
    }
}

pub fn calculate_duration_between_two_dates(date1: NaiveDate, date2: NaiveDate) -> i64 {
    let duration = date2.signed_duration_since(date1);

    duration.num_days()
}

pub fn format_date_time(datetime: DateTime<Utc>) -> String {
    datetime.format("%A, %B %e, %H:%M:%S %Z").to_string()
}
