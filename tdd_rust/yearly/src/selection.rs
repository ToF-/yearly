use crate::date::Date;
use crate::period::{Period,from_year, from_year_past, from_year_up_to};

pub enum SelectionType { Absolute, Running, ToDate, }

pub struct Selection {
    selection_type: SelectionType,
    period: Period,
}

pub fn selection(selection_type: SelectionType, year: i32, month: Option<u32>) -> Option<Selection> {
    match selection_type {
        SelectionType::Absolute => Some(
            Selection { 
                selection_type: selection_type,
                period: from_year(year),
            }),
        SelectionType::Running => {
            let m = month?;
            let p = from_year_past(year, m)?;
            Some(
                Selection {
                    selection_type: selection_type,
                    period: p
                })
        }
        SelectionType::ToDate => {
            let m = month?;
            let p = from_year_up_to(year, m)?;
            Some(
                Selection {
                    selection_type: selection_type,
                    period: p
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absolute_selection_is_made_from_an_option_year_and_month() {
        match selection(SelectionType::Absolute, 2020, None) {
            Some(selection) => assert_eq!(selection.period, ( Date::from_ymd(2020,1,1) , Date::from_ymd(2020,12,31))),
            None => panic!(),
        }
    }

    #[test]
    fn a_running_selection_is_made_from_an_option_year_and_month() {
        match selection(SelectionType::Running, 2020, Some(4)) {
            Some(selection) => assert_eq!(selection.period, ( Date::from_ymd(2019,5,1) , Date::from_ymd(2020,4,30))),
            None => panic!(),
        }
    }

    #[test]
    fn a_running_selection_is_made_from_an_option_year_up_to_a_month() {
        match selection(SelectionType::ToDate, 2020, Some(4)) {
            Some(selection) => assert_eq!(selection.period, ( Date::from_ymd(2020,1,1) , Date::from_ymd(2020,4,30))),
            None => panic!(),
        }
    }
}
