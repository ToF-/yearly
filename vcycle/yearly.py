import sys
from datetime import date
from datetime import timedelta
from calendar import monthrange
from yearlyselection import Period, YearlySelectionType , yearly_selection , current_year_and_month
from options import parse_options
from transaction import get_transactions, summarize, within_period
from report import summary_lines, report

yearly_selection = yearly_selection(parse_options())
transactions = get_transactions('../data/trx.csv')
print(report(transactions, yearly_selection))
