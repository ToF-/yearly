from datetime import datetime
from collections import namedtuple
Period = namedtuple("Period","start end")

def show_date(date):
    return datetime.strftime(date, '%Y-%b-%d')

def show_period(period):
    return '{0} {1}'.format(show_date(period.start), show_date(period.end))
