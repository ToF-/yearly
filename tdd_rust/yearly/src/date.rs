use chrono::{Duration};
pub use chrono::NaiveDate as Date;


pub fn a_day_earlier(date: Date) -> Option<Date> {
    date.checked_sub_signed(Duration::days(1))
}

pub fn end_of_month(year: i32, month: u32) -> Option<Date> {
    match month {
        12 => Some(Date::from_ymd(year,12,31)),
        _  => a_day_earlier(Date::from_ymd(year, month+1, 1)),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate as Date};

    #[test]
    fn a_day_earlier_should_be_equal_to_a_date_minus_1_day() {
        let feb29: Date = Date::from_ymd(2020,02,29);
        let mar01: Date = Date::from_ymd(2020,03,01);
        assert_eq!(a_day_earlier(mar01), Some(feb29));
    }

    #[test]
    fn given_a_year_and_a_month_find_end_of_month() {
        assert_eq!(end_of_month(2020,02), Some(Date::from_ymd(2020,02,29)));
        assert_eq!(end_of_month(2020,01), Some(Date::from_ymd(2020,01,31)));
        assert_eq!(end_of_month(2020,12), Some(Date::from_ymd(2020,12,31)));
    }

}
