#!/bin/bash
# Toggle hypridle on/off

if pgrep -x "hypridle" > /dev/null; then
    pkill hypridle
    notify-send -u low "◉ APERTURE SCIENCE" "IDLE TRACKING: DISABLED\nFacility will remain active."
else
    hypridle &disown
    notify-send -u low "◉ APERTURE SCIENCE" "IDLE TRACKING: ENABLED\nFacility sleep mode armed."
fi
