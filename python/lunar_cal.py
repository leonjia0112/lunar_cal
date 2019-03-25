#!/bin/python
import argparse
import datetime
import json

# parser = argparse.ArgumentParser(description='Lunar Calender Calculator')
# parser.add_argument('-t', '--today', action='store_true', help='Show today\'s date in Lunar Calender')
# 
# parser.parse_args('--today'.split())


def date_to_nth_day(year, month, day):
    if (year % 4 == 0 and year % 100 != 0) or year % 400 == 0:
        month_len = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    else:
        month_len = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    nth_day = 0
    for i in range(month - 1):
        nth_day = nth_day + month_len[i]
    return nth_day + day

def get_lunar_day(date_map, year, month, day):
    day = date_to_nth_day(year, month, day)
    lunar_day = int(date_map.get(str(year))[0])

    if date_map.get(str(year)) == None or \
            (lunar_day > day and date_map.get(str(year-1)) == None):
        return -1

    if lunar_day > day:
        year = year - 1
        lunar_day = int(date_map.get(str(year))[0])
        if (year % 4 == 0 and year % 100 != 0) or year % 400 == 0:
            day = day + 366
        else:
            day = day + 365
    
    lunar_month = 0;
    lunar_month_len = date_map.get(str(year))[1]
    for i in range(len(lunar_month_len)):
        if lunar_month_len[i] == '1':
            lunar_day = lunar_day + 30
        elif lunar_month_len[i] == '0':
            lunar_day = lunar_day + 29
        elif lunar_month_len[i] == '-':
            lunar_month = lunar_month - 1

        lunar_month = lunar_month + 1
        if lunar_day > day:
            lunar_day = lunar_day - 29 if lunar_month_len[i] == '0' else lunar_day - 30 
            break;

    lunar_day = day - lunar_day + 1
    return (lunar_month, lunar_day)

def main():
    d = datetime.datetime.today();
    print("Today is {}/{}/{}.".format(d.year, d.month, d.day))
    date_map = {}
    with open('../data/date_map.json', 'r') as json_f:
        date_map = json.load(json_f)

    (lunar_month, lunar_day) = get_lunar_day(date_map, d.year, d.month, d.day)
    print("Today's is Lunar Date {}-{}.".format(lunar_month, lunar_day))    

if __name__ == "__main__":
    main()
            
