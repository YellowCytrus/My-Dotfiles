#!/bin/bash
# Toggle script for waybar integration
# Place this in ~/.config/waybar/scripts/toggle-schedule.sh

if pgrep -x "sfu-schedule" > /dev/null; then
    pkill -x "sfu-schedule"
else
    sfu-schedule &
fi
