from collections import namedtuple
from datetime import datetime
import csv
from period import Period

Transaction = namedtuple("Transaction", "date category amount")

def get_transactions(filename):
    transactions = []
    with open(filename) as csvfile:
        transaction_reader = csv.reader(csvfile, delimiter=',', quotechar='"')
        header = False
        for row in transaction_reader:
            if (not header):
                header = True
                continue
            transaction_date = datetime.strptime(row[0], '%Y-%m-%d')
            transaction_category = row[2]
            index = row[3].find('.')
            integer_part = int(row[3][0:index])
            transaction_amount = int(row[3][0:index]) * 100 + int(row[3][index+1:])
            transactions.append(Transaction(transaction_date, transaction_category, transaction_amount))
    return transactions

def summarize(transactions):
    summary = {}
    for transaction in transactions:
        if(not transaction.category in summary):
            summary[transaction.category] = transaction.amount
        else:
            summary[transaction.category] += transaction.amount
    return summary

def within_period(transactions, period):
    return [transaction for transaction in transactions if period.start <= transaction.date and transaction.date <= period.end]
