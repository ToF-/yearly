import sys
from datetime import date
from datetime import date
from datetime import timedelta
from calendar import monthrange
from yearlyselection import Period, YearlySelectionType , yearly_selection , current_year_and_month
from options import parse_options
from transaction import get_transactions, summarize, within_period

yearly_selection = yearly_selection(parse_options())
print(yearly_selection)
transactions = within_period(get_transactions('../data/trx.csv'),yearly_selection.current)
print(transactions)
