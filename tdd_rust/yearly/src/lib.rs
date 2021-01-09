mod date {

    use chrono::{Date,Duration,Utc};

    pub fn a_day_earlier(date: Date<Utc>) -> Option<Date<Utc>> {
        date.checked_sub_signed(Duration::days(1))
    }

    pub fn within(period: (Date<Utc>, Date<Utc>), date: Date<Utc>) -> bool {
        let (start, end) = period;
        date >= start && date <= end 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use date::*;
    use chrono::{Duration,Date,TimeZone,Utc};

    #[test]
    fn a_date_should_be_equal_to_itself() {
        assert_eq!(Utc.ymd(2020,02,29), Utc.ymd(2020,02,29));
    }

    #[test]
    fn a_day_earlier_should_be_equal_to_a_date_minus_1_day() {
        assert_eq!(a_day_earlier(Utc.ymd(2020,02,29)), Some(Utc.ymd(2020,02,28)));
    }

    #[test]
    fn a_date_can_be_within_a_period() {
        let period = (Utc.ymd(2020, 01, 01), Utc.ymd(2020,12,31));
        assert_eq!(within(period,Utc.ymd(2020,03,31)),true);
        assert_eq!(within(period,Utc.ymd(2019,03,31)),false);
        assert_eq!(within(period,Utc.ymd(2021,03,31)),false);
        assert_eq!(within(period,Utc.ymd(2020,01,01)),true);
        assert_eq!(within(period,Utc.ymd(2020,12,31)),true);
    }
}
