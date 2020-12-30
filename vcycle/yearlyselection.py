from collections import namedtuple
from datetime import date
from datetime import timedelta
from enum import Enum

Period = namedtuple("Period","start end")
YearlySelection = namedtuple("YearlySelection", "current previous")

def current_year_and_month():
    today = date.today()
    return [today.year, today.month]

def last_day_of_month(date):
    if date.month == 12:
        return date.replace(day=31)
    return date.replace(month=date.month+1, day=1) - timedelta(days=1)

class YearlySelectionType(Enum):
    ABSOLUTE = 0
    RUNNING  = 1
    TODATE   = 2

def yearly_selection(args):
    current_year = args.current_month[0]
    current_month= args.current_month[1]
    if args.yearly_selection_type != YearlySelectionType.ABSOLUTE:
        current_end = date(current_year, current_month, last_day_of_month(date(current_year, current_month, 1)).day)
    else:
        current_end = date(current_year, 12, 31)

    if args.yearly_selection_type != YearlySelectionType.TODATE:
        previous_end = date(current_end.year-1, current_end.month, current_end.day)
    else:
        previous_end = date(current_end.year-1, 12,31)
    current_start = previous_end + timedelta(days=1)
    previous_start = date(current_start.year-1, current_start.month, current_start.day)
    return YearlySelection(Period(current_start, current_end), Period(previous_start, previous_end))

