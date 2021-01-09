use chrono::{Duration,Date,TimeZone,Utc};
#[cfg(test)]
mod tests {
    use super::*;

fn a_day_earlier(date: Date<Utc>) -> Option<Date<Utc>> {
    date.checked_sub_signed(Duration::days(1))
}
    #[test]
    fn a_date_should_be_equal_to_itself() {
        assert_eq!(Utc.ymd(2020,02,29), Utc.ymd(2020,02,29));
    }

    #[test]
    fn a_day_earlier_should_be_equal_to_a_date_minus_1_day() {
        assert_eq!(a_day_earlier(Utc.ymd(2020,02,29)), Some(Utc.ymd(2020,02,28)));
    }
}
