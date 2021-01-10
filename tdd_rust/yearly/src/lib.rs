mod date {

    use chrono::{Date,Duration,TimeZone,Utc};

    pub fn a_day_earlier(date: Date<Utc>) -> Option<Date<Utc>> {
        date.checked_sub_signed(Duration::days(1))
    }

    pub fn within(period: (Date<Utc>, Date<Utc>), date: Date<Utc>) -> bool {
        let (start, end) = period;
        date >= start && date <= end
    }

    pub fn end_of_month(year: i32, month: u32) -> Option<Date<Utc>> {
        match month {
            12 => Some(Utc.ymd(year,12,31)),
            _  => a_day_earlier(Utc.ymd(year, month+1, 1)),
        }
    }
}

mod transaction {

    use chrono::{Date,Utc};
    use std::collections::HashMap;

pub struct Transaction {
    pub date: Date<Utc>,
    pub category: String,
    pub label: String,
    pub amount: i64,
}

pub struct Total {
    pub category: String,
    pub amount: i64,
}

pub fn total_per_category(transactions: Vec<Transaction>) -> Vec<Total> {
    let mut totals = HashMap::<String,i64>::new();
    transactions.iter().for_each( | transaction |
                                  {
                                         let amount:i64 = *totals.entry(transaction.category.clone()).or_insert(0);
                                         totals.insert(transaction.category.clone(), amount + transaction.amount);
                                     });
    let mut result = Vec::<Total>::new();
    totals.iter().for_each( | (category,amount) | result.push(Total { category: category.clone(), amount: *amount, }));
    result
}

}
#[cfg(test)]
mod tests_transaction {
    use super::*;
    use date::*;
    use transaction::*;
    use chrono::{Duration,Date,TimeZone,Utc};

    #[test]
    fn total_per_category_on_an_empty_list_should_yield_an_empty_list() {
        let transactions = Vec::<Transaction>::new();
        let totals = total_per_category(transactions);
        assert_eq!(totals.len(), 0);
    }

    #[test]
    fn total_per_category_on_a_single_transaction_should_yield_the_transaction_amount() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction {
            date: Utc.ymd(2020,02,29),
            label: "some groceries".to_string(),
            category: "Groceries".to_string(),
            amount: 4807,
        });

        let totals = total_per_category(transactions);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amount, 4807);
    }

    #[test]
    fn total_on_one_category_for_everal_transactions_should_yield_the_category_total() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction {
            date: Utc.ymd(2020,02,29),
            label: "some groceries".to_string(),
            category: "Groceries".to_string(),
            amount: 4807,
        });
        transactions.push(Transaction {
            date: Utc.ymd(2020,03,20),
            label: "other groceries".to_string(),
            category: "Groceries".to_string(),
            amount: 10000,
        });

        let totals = total_per_category(transactions);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amount, 14807);
    }
}
#[cfg(test)]
mod tests_date {
    use super::*;
    use date::*;
    use chrono::{Date,TimeZone,Utc};

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

    #[test]
    fn given_a_year_and_a_month_find_end_of_month() {
        assert_eq!(end_of_month(2020,02), Some(Utc.ymd(2020,02,29)));
        assert_eq!(end_of_month(2020,01), Some(Utc.ymd(2020,01,31)));
        assert_eq!(end_of_month(2020,12), Some(Utc.ymd(2020,12,31)));
    }

}
