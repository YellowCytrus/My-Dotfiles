#!/bin/bash
# Toggle global window transparency on/off

CURRENT=$(hyprctl getoption decoration:active_opacity -j | grep -o '"float": [0-9.]*' | awk '{print $2}')

if (( $(echo "$CURRENT < 1.0" | bc -l) )); then
    # Полностью убираем прозрачность: opacity 1, без blur и теней (они дают эффект просвечивания)
    hyprctl --batch "\
        keyword decoration:active_opacity 1.0;\
        keyword decoration:inactive_opacity 1.0;\
        keyword decoration:fullscreen_opacity 1.0;\
        keyword decoration:dim_inactive false;\
        keyword decoration:shadow:enabled false"
    hyprctl keyword "windowrule opacity 1 override 1 override 1 override, ^(.*)$"
    # Применяем opaque к уже открытым окнам (windowrule не влияет на существующие)
    while read -r addr; do
        hyprctl dispatch setprop "address:$addr" opaque 1 2>/dev/null
    done < <(hyprctl clients -j | jq -r '.[].address')
    notify-send -u low "◉ APERTURE SCIENCE" "TRANSPARENCY PROTOCOL: DISABLED"
else
    # Снимаем opaque с всех окон перед reload
    while read -r addr; do
        hyprctl dispatch setprop "address:$addr" opaque 0 2>/dev/null
    done < <(hyprctl clients -j | jq -r '.[].address')
    # Восстанавливаем прозрачность: reload убирает override-правило и загружает настройки из конфига
    hyprctl reload
    notify-send -u low "◉ APERTURE SCIENCE" "TRANSPARENCY PROTOCOL: ENABLED"
fi
