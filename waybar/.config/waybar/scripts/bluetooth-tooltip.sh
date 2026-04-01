#!/bin/bash

# Проверяем, включён ли Bluetooth
if ! bluetoothctl show | grep -q "Powered: yes"; then
    echo "<span font='13'>Bluetooth</span>\n<span foreground='#f7768e'>Выключен</span>"
    exit 0
fi

# Получаем ТОЛЬКО устройства с именами (без MAC, без интерфейсов)
mapfile -t devices < <(bluetoothctl devices Connected | grep -E "Device" | awk '{print $3" "$4" "$5" "$6" "$7" "$8}' | sed 's/ $//')

# Фильтруем пустые
filtered=()
for dev in "${devices[@]}"; do
    [[ -n "$dev" ]] && filtered+=("• $dev")
done

count=${#filtered[@]}

if [[ $count -gt 0 ]]; then
    devices_list=$(printf '%s\n' "${filtered[@]}")
    echo "<span font='13'>Bluetooth</span>\n<span foreground='#98c379'>$devices_list</span>\n<span foreground='#7aa2f7'>$count подключено</span>"
else
    echo "<span font='13'>Bluetooth</span>\n<span foreground='#565f89'>Нет устройств</span>\n<span foreground='#7aa2f7'>0 подключено</span>"
fi