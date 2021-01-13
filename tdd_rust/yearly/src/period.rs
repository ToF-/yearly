use crate::date::{Date,end_of_month};

pub type Period = (Date, Date);

pub fn within(period: Period, date: Date) -> bool {
    let (start, end) = period;
    date >= start && date <= end
}

pub fn from_year(year: i32) -> Option<Period> {
    if year > 0 { 
        Some((Date::from_ymd(year, 1, 1), Date::from_ymd(year, 12, 31)))
    }
    else {
        None
    }
}

pub fn from_year_up_to(year: i32, month: u32) -> Option<Period> {
    let start: Date = Date::from_ymd(year, 1, 1);
    let end = end_of_month(year, month)?;
    Some((start, end))
}

pub fn from_year_past(year: i32, month: u32) -> Option<Period> {
    let end = end_of_month(year, month)?;
    let start = match month {
        12 => Date::from_ymd(year, 1, 1),
        m  => Date::from_ymd(year-1, m+1, 1),
    };
    Some((start, end))
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

    #[test]
    fn a_period_can_be_created_given_a_year() {
        assert_eq!(from_year(2020), Some((Date::from_ymd(2020,01,01),Date::from_ymd(2020,12,31))));
    }

    #[test]
    fn a_period_from_jan_1st_can_be_created_given_a_year_and_month() {
        assert_eq!(from_year_up_to(2020, 2), Some((Date::from_ymd(2020,01,01),Date::from_ymd(2020,2,29))));

    }

    #[test]
    fn a_period_for_past_twelve_months_can_be_created_given_a_year_and_month() {
        assert_eq!(from_year_past(2020, 2), Some((Date::from_ymd(2019,03,01),Date::from_ymd(2020,2,29))));
        assert_eq!(from_year_past(2020, 12), Some((Date::from_ymd(2020,1,1),Date::from_ymd(2020,12,31))));
        assert_eq!(from_year_past(2021, 1), Some((Date::from_ymd(2020,2,1),Date::from_ymd(2021,1,31))));

    }


}
