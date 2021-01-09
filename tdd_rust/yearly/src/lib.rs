use chrono::{TimeZone,Utc};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_date_should_be_equal_to_itself() {
        assert_eq!(Utc.ymd(2020,02,29), Utc.ymd(2020,02,29));
    }
}
