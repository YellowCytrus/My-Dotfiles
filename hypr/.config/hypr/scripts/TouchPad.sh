#!/bin/bash
# /* ---- 💫 https://github.com/JaKooLit 💫 ---- */  ##
# For disabling/enabling touchpad.
# Uses the correct touchpad device name: ELAN0712:00 04F3:30FD Touchpad

notif="$HOME/.config/swaync/images/ja.png"
export STATUS_FILE="$XDG_RUNTIME_DIR/touchpad.status"
TOUCHPAD="ELAN0712:00 04F3:30FD Touchpad"

enable_touchpad() {
    printf "true" > "$STATUS_FILE"
    notify-send -u low -i "$notif" "Enabling" "Touchpad"
    hyprctl keyword device:"$TOUCHPAD":enabled true
}

disable_touchpad() {
    printf "false" > "$STATUS_FILE"
    notify-send -u low -i "$notif" "Disabling" "Touchpad"
    hyprctl keyword device:"$TOUCHPAD":enabled false
}

if ! [ -f "$STATUS_FILE" ]; then
    enable_touchpad
else
    if [ "$(cat "$STATUS_FILE")" = "true" ]; then
        disable_touchpad
    elif [ "$(cat "$STATUS_FILE")" = "false" ]; then
        enable_touchpad
    fi
fi
