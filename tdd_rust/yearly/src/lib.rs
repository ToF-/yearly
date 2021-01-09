use chrono::{TimeZone,Utc};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_expenses_per_category() {
        assert_eq!(Utc.ymd(2020,02,29), Utc.ymd(2020,02,29));
    }
}
