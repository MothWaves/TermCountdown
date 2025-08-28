#!/usr/bin/env python3

from datetime import datetime, timedelta, timezone
from math import floor
from time import sleep
import os


def main(target_datetime):
    while True:
        clear_screen()
        current_time = datetime.now(timezone(timedelta(hours=1)))
        delta = target_datetime - current_time
        delta_hours = floor(delta.seconds / 3600)
        removed_seconds = delta_hours * 3600
        delta_minutes = floor((delta.seconds - removed_seconds) / 60)
        removed_seconds += delta_minutes * 60
        delta_seconds = delta.seconds - removed_seconds
        print("{} days".format(delta.days))
        print("{} hours".format(delta_hours))
        print("{} minutes".format(delta_minutes))
        print("{} seconds".format(delta_seconds))
        sleep(1)


def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

if __name__ == "__main__":
    target_datetime = datetime.fromisoformat('2025-08-28T17:00:00+01:00')
    main(target_datetime)
