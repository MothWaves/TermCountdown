#!/usr/bin/env python3

from datetime import datetime, timedelta, timezone
from math import floor
from time import sleep
import os

class Countdown:
    def __init__(self, days, hours, minutes, seconds):
        self.days = days
        self.hours = hours
        self.minutes = minutes
        self.seconds = seconds

def main(target_datetime):
    while True:
        clear_screen()
        current_time = get_current_datetime()
        count = get_countdown(current_time, target_datetime)
        print("{} days".format(count.days))
        print("{} hours".format(count.hours))
        print("{} minutes".format(count.minutes))
        print("{} seconds".format(count.seconds))
        sleep(1)


def get_countdown(start_date, end_date):
        delta = end_date - start_date
        delta_hours = floor(delta.seconds / 3600)
        removed_seconds = delta_hours * 3600
        delta_minutes = floor((delta.seconds - removed_seconds) / 60)
        removed_seconds += delta_minutes * 60
        delta_seconds = delta.seconds - removed_seconds
        countdown = Countdown(delta.days, delta_hours, delta_minutes, delta_seconds)
        return countdown

def get_current_datetime():
    return datetime.now(timezone(timedelta(hours=1)))

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

if __name__ == "__main__":
    target_datetime = datetime.fromisoformat('2025-12-25T08:00:00+01:00')
    main(target_datetime)
