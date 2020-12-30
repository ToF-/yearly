import sys
from datetime import date
from datetime import date
from datetime import timedelta
from calendar import monthrange
from yearlyselection import Period, YearlySelectionType , yearly_selection , current_year_and_month
from options import parse_options
print (yearly_selection(parse_options()))


today = date(2020,2,1) # date.today()
start = date(today.year, today.month, 1)
end = start + timedelta(days = 365) + timedelta(days=-1)

current_period=Period(start, end)
previous_period=Period(date(current_period.start.year-1,current_period.start.month,1), start+timedelta(days=-1))

