#!/bin/bash
# Dynamic wallpaper — shifts based on time of day
# Uses swww for smooth transitions

WALL_DIR="$HOME/Pictures/wallpapers"
TRANSITION_TYPE="fade"
TRANSITION_DURATION=2
TRANSITION_FPS=60

get_wallpaper_for_hour() {
    local hour=$1

    if [ "$hour" -ge 6 ] && [ "$hour" -lt 10 ]; then
        # Morning — warm sunset tones
        echo "$WALL_DIR/Fantasy - Sunset.png"
    elif [ "$hour" -ge 10 ] && [ "$hour" -lt 17 ]; then
        # Day — Aperture Science
        echo "$WALL_DIR/Aperture-Science.png"
    elif [ "$hour" -ge 17 ] && [ "$hour" -lt 20 ]; then
        # Evening — sunset
        echo "$WALL_DIR/Sunset-Forrest.png"
    elif [ "$hour" -ge 20 ] && [ "$hour" -lt 23 ]; then
        # Night — city
        echo "$WALL_DIR/City-Night.png"
    else
        # Late night — dark
        echo "$WALL_DIR/City-Rainy-Night.png"
    fi
}

apply_wallpaper() {
    local wallpaper="$1"
    if [ -f "$wallpaper" ]; then
        swww img "$wallpaper" \
            --transition-type "$TRANSITION_TYPE" \
            --transition-duration "$TRANSITION_DURATION" \
            --transition-fps "$TRANSITION_FPS"
    fi
}

case "${1:-auto}" in
    auto)
        HOUR=$(date +%H)
        HOUR=$((10#$HOUR))
        WALLPAPER=$(get_wallpaper_for_hour $HOUR)
        apply_wallpaper "$WALLPAPER"
        ;;
    loop)
        LAST_WALL=""
        while true; do
            HOUR=$(date +%H)
            HOUR=$((10#$HOUR))
            WALLPAPER=$(get_wallpaper_for_hour $HOUR)
            if [ "$WALLPAPER" != "$LAST_WALL" ]; then
                apply_wallpaper "$WALLPAPER"
                LAST_WALL="$WALLPAPER"
            fi
            sleep 300
        done
        ;;
    set)
        apply_wallpaper "$2"
        ;;
    *)
        echo "Usage: $0 {auto|loop|set <path>}"
        ;;
esac
