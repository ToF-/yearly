from transaction import summarize, within_period
from collections import namedtuple
from period import show_period, show_period_month
from yearlyselection import YearlySelectionType

SummaryAmount = namedtuple("SummaryAmount", "current_amount previous_amount")

def summary_lines(transactions, yearly_selection):
    current_summary = summarize(within_period(transactions, yearly_selection.current))
    previous_summary= summarize(within_period(transactions, yearly_selection.previous))
    categories = list(current_summary.keys()) + list(previous_summary.keys())
    summary_lines = {}
    for category in sorted(categories):
        current_amount = current_summary.get(category, 0)
        previous_amount = previous_summary.get(category, 0)
        summary_lines[category] = SummaryAmount(current_amount, previous_amount)

    return summary_lines

def summary_header(yearly_selection):
    if(yearly_selection.yearly_selection_type == YearlySelectionType.ABSOLUTE):
        period = (show_period_month(yearly_selection.current),show_period_month(yearly_selection.previous))
    else:
        period = (show_period(yearly_selection.current), show_period(yearly_selection.previous))
    return 'Yearly report for all categories : %s | %s' % period

def summary_total(summary_lines):
    current_total = 0
    previous_total = 0
    for key in summary_lines:
        summary_amount = summary_lines[key]
        current_total += summary_amount.current_amount
        previous_total += summary_amount.previous_amount
    return '{:48s} : {:s} | {:s}'.format('TOTAL', show_amount(current_total), show_amount(previous_total))

def show_amount(amount):
    return '{:9.2f}'.format(amount/100)

def show_summary_lines(lines):
    result = ''
    for key in lines.keys():
        line = '{:48s} : {:s} | {:s}'.format(key, show_amount(lines[key].current_amount), show_amount(lines[key].previous_amount))
        result += line + '\n'
    return result

def report(transactions, yearly_selection):
    lines = summary_lines(transactions, yearly_selection)
    return summary_header(yearly_selection) + '\n' + show_summary_lines(lines) + summary_total(lines)
