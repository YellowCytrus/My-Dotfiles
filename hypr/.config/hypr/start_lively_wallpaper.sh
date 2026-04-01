#!/bin/bash
# Путь к вашему видеофайлу
VIDEO_FILE="live_wallpaper.mp4"

# Запуск mpv в цикле без рамки, без управления, и в фоновом режиме
mpv --loop --no-audio --wid=$(xprop -root _NET_WM_PID | cut -d ' ' -f 3) --no-border --no-osc --no-input-default-bindings --geometry=100%:100% $VIDEO_FILE &
