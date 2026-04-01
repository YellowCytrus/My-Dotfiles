#!/bin/bash

STATE="/tmp/hypr_blur_toggle"

if [ -f "$STATE" ]; then
    hyprctl --batch "\
        keyword decoration:blur:size 10;\
        keyword decoration:blur:passes 3"
    rm "$STATE"
    notify-send -u low "◉ APERTURE SCIENCE" "BLUR PROTOCOL: DEFAULT (10/3)"
else
    hyprctl --batch "\
        keyword decoration:blur:size 1;\
        keyword decoration:blur:passes 1"
    touch "$STATE"
    notify-send -u low "◉ APERTURE SCIENCE" "BLUR PROTOCOL: MINIMAL (1/1)"
fi
