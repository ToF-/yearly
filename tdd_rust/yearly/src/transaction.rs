use crate::{period::within,period::Period};
use crate::date::Date;
use std::collections::HashMap;

pub struct Transaction {
    pub date: Date,
    pub category: String,
    pub label: String,
    pub amount: i64,
}
#[derive(PartialEq,Eq,Ord,PartialOrd)]
pub struct Total {
    pub category: String,
    pub amounts: (i64, i64),
}
pub enum Column {
    Current,
    Previous
}

pub fn total_per_category(transactions: Vec<Transaction>, period: Column) -> Vec<Total> {

    let mut totals = HashMap::<String,(i64,i64)>::new();
    transactions.iter().for_each( | transaction |
                                  {
                                      let (current_amount, previous_amount) = *totals.entry(transaction.category.clone()).or_insert((0,0));
                                      let amounts = match &period {
                                          Column::Current =>  (current_amount + transaction.amount, previous_amount),
                                          Column::Previous => (current_amount, previous_amount + transaction.amount),
                                      };
                                      totals.insert(transaction.category.clone(), amounts);
                                  });
    let mut result = Vec::<Total>::new();
    totals.iter().for_each( | (category,amounts) | result.push(Total { category: category.clone(), amounts: *amounts, }));
    result.sort();
    result
}

pub fn from_period(mut transactions: Vec<Transaction>, period: Period) -> Vec<Transaction> {
    transactions.retain(|transaction| within(period, transaction.date));
    transactions
}
#[cfg(test)]
mod tests_transaction {
    use super::*;

    #[test]
    fn total_per_category_on_an_empty_list_should_yield_an_empty_list() {
        let transactions = Vec::<Transaction>::new();
        let totals = total_per_category(transactions, Column::Current);
        assert_eq!(totals.len(), 0);
    }

    #[test]
    fn total_per_category_on_a_single_transaction_should_yield_the_transaction_amount() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction {
            date: Date::from_ymd(2020,02,29),
            label: "some groceries".to_string(),
            category: "Groceries".to_string(),
            amount: 4807,
        });

        let totals = total_per_category(transactions, Column::Current);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (4807,0));
    }

    #[test]
    fn total_on_one_category_for_several_transactions_should_yield_the_category_total() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Date::from_ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Date::from_ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        let totals = total_per_category(transactions, Column::Current);
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (14807,0));
    }

    #[test]
    fn total_on_several_categories_for_several_transactions_should_yield_the_total_per_category_on_current_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Date::from_ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Date::from_ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Date::from_ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Date::from_ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });

        let totals = total_per_category(transactions, Column::Current);
        assert_eq!(totals.len(), 2);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (14807,0));
        assert_eq!(totals[1].category, "Taxes");
        assert_eq!(totals[1].amounts, (22000,0));
    }

    #[test]
    fn total_on_several_categories_for_several_transactions_should_yield_the_total_per_category_on_previous_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Date::from_ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Date::from_ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Date::from_ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Date::from_ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });

        let totals = total_per_category(transactions, Column::Previous);
        assert_eq!(totals.len(), 2);
        assert_eq!(totals[0].category, "Groceries");
        assert_eq!(totals[0].amounts, (0,14807));
        assert_eq!(totals[1].category, "Taxes");
        assert_eq!(totals[1].amounts, (0,22000));
    }

    #[test]
    fn transactions_should_be_selected_given_a_period() {
        let mut transactions = Vec::<Transaction>::new();
        transactions.push(Transaction { date: Date::from_ymd(2020,02,29), label: "some groceries".to_string(), category: "Groceries".to_string(), amount: 4807, });
        transactions.push(Transaction { date: Date::from_ymd(2020,03,20), label: "other groceries".to_string(), category: "Groceries".to_string(), amount: 10000, });

        transactions.push(Transaction { date: Date::from_ymd(2020,01,29), label: "some taxes".to_string(), category: "Taxes".to_string(), amount: 2000, });
        transactions.push(Transaction { date: Date::from_ymd(2020,04,20), label: "other taxes".to_string(), category: "Taxes".to_string(), amount: 20000, });
        let period = (Date::from_ymd(2020, 01, 01), Date::from_ymd(2020,02,29));
        let selection = from_period(transactions, period);
        assert_eq!(selection.len(), 2);
        assert_eq!(selection[0].label, "some groceries");
        assert_eq!(selection[1].label, "some taxes");
    }
}
