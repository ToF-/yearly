use crate::date::Date;

pub type Period = (Date, Date);

pub fn within(period: Period, date: Date) -> bool {
    let (start, end) = period;
    date >= start && date <= end
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_date_can_be_within_a_period() {
        let period = (Date::from_ymd(2020, 01, 01), Date::from_ymd(2020,12,31));
        assert_eq!(within(period,Date::from_ymd(2020,03,31)),true);
        assert_eq!(within(period,Date::from_ymd(2019,03,31)),false);
        assert_eq!(within(period,Date::from_ymd(2021,03,31)),false);
        assert_eq!(within(period,Date::from_ymd(2020,01,01)),true);
        assert_eq!(within(period,Date::from_ymd(2020,12,31)),true);
    }
}
