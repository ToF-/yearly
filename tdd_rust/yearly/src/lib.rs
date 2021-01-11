#[allow(dead_code)]
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

use std::collections::HashMap;

pub struct Transaction {
    pub date: Date<Utc>,
    pub category: String,
    pub label: String,
    pub amount: i64,
}
#[derive(PartialEq,Eq,Ord,PartialOrd)]
pub struct Total {
    pub category: String,
    pub amounts: (i64, i64),
}
pub enum Period {
    Current,
    Previous
}

pub fn total_per_category(transactions: Vec<Transaction>, period: Period) -> Vec<Total> {

    let mut totals = HashMap::<String,(i64,i64)>::new();
    transactions.iter().for_each( | transaction |
                                  {
                                      let (current_amount, previous_amount) = *totals.entry(transaction.category.clone()).or_insert((0,0));
                                      let amounts = match &period {
                                          Period::Current =>  (current_amount + transaction.amount, previous_amount),
                                          Period::Previous => (current_amount, previous_amount + transaction.amount),
                                      };
                                      totals.insert(transaction.category.clone(), amounts);
                                  });
    let mut result = Vec::<Total>::new();
    totals.iter().for_each( | (category,amounts) | result.push(Total { category: category.clone(), amounts: *amounts, }));
    result.sort();
    result
}

pub fn from_period(mut transactions: Vec<Transaction>, period: (Date<Utc>, Date<Utc>)) -> Vec<Transaction> {
    transactions.retain(|transaction| within(period, transaction.date));
    transactions
}
#[cfg(test)]
mod tests_transaction {
    use super::*;
    use chrono::{TimeZone,Utc};

    #[test]
    fn total_per_category_on_an_empty_list_should_yield_an_empty_list() {
        let transactions = Vec::<Transaction>::new();
        let totals = total_per_category(transactions, Period::Current);
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

        let totals = total_per_category(transactions, Period::Current);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (4807,0));
    }

    #[test]
    fn total_on_one_category_for_several_transactions_should_yield_the_category_total() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Utc.ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Utc.ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        let totals = total_per_category(transactions, Period::Current);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (14807,0));
    }

    #[test]
    fn total_on_several_categories_for_several_transactions_should_yield_the_total_per_category_on_current_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Utc.ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Utc.ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Utc.ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Utc.ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });

        let totals = total_per_category(transactions, Period::Current);
        assert_eq!(totals.len(), 2);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (14807,0));
        assert_eq!(totals[1].category, "Taxes");
        assert_eq!(totals[1].amounts, (22000,0));
    }

    #[test]
    fn total_on_several_categories_for_several_transactions_should_yield_the_total_per_category_on_previous_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Utc.ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Utc.ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Utc.ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Utc.ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });

        let totals = total_per_category(transactions, Period::Previous);
        assert_eq!(totals.len(), 2);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (0,14807));
        assert_eq!(totals[1].category, "Taxes");
        assert_eq!(totals[1].amounts, (0,22000));
    }

    #[test]
    fn transactions_should_be_selected_given_a_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Utc.ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Utc.ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Utc.ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Utc.ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });
        let period = (Utc.ymd(2020, 01, 01), Utc.ymd(2020,02,29));
        let selection = from_period(transactions, period);
        assert_eq!(selection.len(), 2);
        assert_eq!(selection[0].label, "some groceries");
        assert_eq!(selection[1].label, "some taxes");
    }
}
#[cfg(test)]
mod tests_date {
    use super::*;
    use chrono::{TimeZone,Utc};

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
