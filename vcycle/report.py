from transaction import summarize, within_period
from collections import namedtuple
from period import show_period

SummaryAmount = namedtuple("SummaryAmount", "current_amount previous_amount")

def summary_lines(transactions, yearly_selection):
    current_summary = summarize(within_period(transactions, yearly_selection.current))
    previous_summary= summarize(within_period(transactions, yearly_selection.previous))
    categories = list(current_summary.keys()) + list(previous_summary.keys())
    summary_lines = {}
    for category in categories:
        current_amount = current_summary.get(category, 0)
        previous_amount = previous_summary.get(category, 0)
        summary_lines[category] = SummaryAmount(current_amount, previous_amount)

    return summary_lines

def summary_header(yearly_selection):
    return 'Yearly report for all categories : %s | %s' % (show_period(yearly_selection.current), show_period(yearly_selection.previous))

def summary_total(summary_lines):
    current_total = 0
    previous_total = 0
    for key in summary_lines:
        summary_amount = summary_lines[key]
        current_total += summary_amount.current_amount
        previous_total += summary_amount.previous_amount
    return '{:50s} {:s} | {:s}'.format('TOTAL', show_amount(current_total), show_amount(previous_total))

def show_amount(amount):
    return '{:10.2f}'.format(amount/100)

def show_summary_lines(lines):
    result = ''
    for key in lines.keys():
        line = '{:50s} {:s} | {:s}'.format(key, show_amount(lines[key].current_amount), show_amount(lines[key].previous_amount))
        result += line + '\n'
    return result

def report(transactions, yearly_selection):
    lines = summary_lines(transactions, yearly_selection)
    return summary_header(yearly_selection) + '\n' + show_summary_lines(lines) + summary_total(lines)
