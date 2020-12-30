from yearlyselection import Period, YearlySelectionType , yearly_selection , current_year_and_month
import argparse
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

def parse_options():
    return parser.parse_args()
