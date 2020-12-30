import sys
from datetime import date
from datetime import date
from datetime import timedelta
import argparse
from calendar import monthrange
from yearlyselection import Period, YearlySelectionType , yearly_selection , current_year_and_month

parser = argparse.ArgumentParser(description = "yearly report")
parser.add_argument('-a'
                   , dest='yearly_selection_type'
                   , action='store_const'
                   , const=YearlySelectionType.ABSOLUTE
                   , default=YearlySelectionType.ABSOLUTE
                   , help = 'compare a full year to the previous year')
parser.add_argument('-r'
                   , dest='yearly_selection_type'
                   , action='store_const'
                   , const=YearlySelectionType.RUNNING
                   , default=YearlySelectionType.ABSOLUTE
                   , help = 'compare the last twelve months to the previous twelve months')
parser.add_argument('-t'
                   , dest='yearly_selection_type'
                   , action='store_const'
                   , const=YearlySelectionType.TODATE
                   , default=YearlySelectionType.ABSOLUTE
                   , help = 'compare the current year up to the current months to the previous full year')
parser.add_argument('current_month'
                   , nargs='*'
                   , type=int
                   , default = current_year_and_month())

args = parser.parse_args()
print (yearly_selection(args))


today = date(2020,2,1) # date.today()
start = date(today.year, today.month, 1)
end = start + timedelta(days = 365) + timedelta(days=-1)

current_period=Period(start, end)
previous_period=Period(date(current_period.start.year-1,current_period.start.month,1), start+timedelta(days=-1))

