#!/bin/bash

# Получаем путь к обоям
WALLPAPER_PATH=$(swww query | grep "current" | awk '{print $NF}')

# Заменяем в шаблоне __WALLPAPER_PATH__ на реальный путь и сохраняем в конфиг
sed "s|__WALLPAPER_PATH__|$WALLPAPER_PATH|g" ~/.config/rofi/config.rasi.template > ~/.config/rofi/config.rasi

# Запускаем Rofi
rofi -show drun -modi drun,filebrowser,run,window
