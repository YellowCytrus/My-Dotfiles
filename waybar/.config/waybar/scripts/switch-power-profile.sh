#!/bin/bash

CURRENT=$(powerprofilesctl get)

case $CURRENT in
    performance)
        powerprofilesctl set balanced
        notify-send "Power Profile" "Switched to Balanced" -i battery-good
        ;;
    balanced)
        powerprofilesctl set power-saver
        notify-send "Power Profile" "Switched to Power Saver" -i battery-low
        ;;
    power-saver)
        powerprofilesctl set performance
        notify-send "Power Profile" "Switched to Performance" -i battery-full-charged
        ;;
    *)
        notify-send "Power Profile" "Unknown: $CURRENT" -i battery-missing
        ;;
esac
