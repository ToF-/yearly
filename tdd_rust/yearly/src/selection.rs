use crate::date::Date;
use crate::period::{Period,from_year, from_year_past, from_year_up_to};

#[derive(PartialEq,Debug)]
pub enum SelectionType { Absolute, Running, ToDate, }

#[derive(PartialEq,Debug)]
pub struct Selection {
    selection_type:  SelectionType,
    current_period:  Period,
    previous_period: Period,
}

pub fn selection(selection_type: SelectionType, year: i32, month: Option<u32>) -> Option<Selection> {
    let (current, previous) = match selection_type {
        SelectionType::Absolute => {
            let current = from_year(year)?;
            let previous = from_year(year-1)?;
            (current, previous)
        },
        SelectionType::Running => {
            let m = month?;
            let current = from_year_past(year, m)?;
            let previous = from_year_past(year-1, m)?;
            (current, previous)
        },
        SelectionType::ToDate => {
            let m = month?;
            let current = from_year_up_to(year, m)?;
            let previous = from_year(year-1)?;
            (current, previous)
        },
    };
    Some(Selection {
        selection_type:  selection_type,
        current_period:  current,
        previous_period: previous, })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absolute_selection_is_made_from_an_option_year_and_month() {
        let selection = selection(SelectionType::Absolute, 2020, None).unwrap();
        assert_eq!(selection.current_period, (Date::from_ymd(2020,1,1), Date::from_ymd(2020,12,31)));
        assert_eq!(selection.previous_period, (Date::from_ymd(2019,1,1), Date::from_ymd(2019,12,31)));
    }

    #[test]
    fn a_running_selection_is_made_from_an_option_year_and_month() {
        let selection = selection(SelectionType::Running, 2020, Some(2)).unwrap();
        assert_eq!(selection.current_period, (Date::from_ymd(2019,3,1), Date::from_ymd(2020,2,29)));
        assert_eq!(selection.previous_period, (Date::from_ymd(2018,3,1), Date::from_ymd(2019,2,28)));
    }

    #[test]
    fn a_running_selection_is_made_from_an_option_year_up_to_a_month() {
        let selection = selection(SelectionType::ToDate, 2020, Some(4)).unwrap();
        assert_eq!(selection.current_period, (Date::from_ymd(2020,1,1), Date::from_ymd(2020,4,30)));
        assert_eq!(selection.previous_period, (Date::from_ymd(2019,1,1), Date::from_ymd(2019,12,31)));
    }

    #[test]
    fn a_selection_of_type_running_with_no_months_cannot_be_defined() {
        let selection = selection(SelectionType::Running, 2020, None);
        assert_eq!(selection, None);
    }
}
