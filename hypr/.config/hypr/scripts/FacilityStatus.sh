#!/bin/bash
# Dynamic Aperture Science facility status based on system load

CPU=$(awk '/^cpu /{u=$2+$4; t=$2+$3+$4+$5+$6+$7+$8; if(t>0) printf "%.0f", u*100/t}' /proc/stat)
MEM=$(free | awk '/Mem:/{printf "%.0f", $3/$2*100}')
TEMP=$(cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null | sort -rn | head -1)
TEMP=$((TEMP / 1000))

if [ "$CPU" -ge 90 ] || [ "$TEMP" -ge 85 ]; then
    TEXT="⚠ CRITICAL LOAD"
    CLASS="critical"
    TIP="CPU: ${CPU}% | MEM: ${MEM}% | TEMP: ${TEMP}°C\nWARNING: System under extreme load"
elif [ "$CPU" -ge 70 ] || [ "$MEM" -ge 85 ]; then
    TEXT="▲ HIGH ACTIVITY"
    CLASS="warning"
    TIP="CPU: ${CPU}% | MEM: ${MEM}% | TEMP: ${TEMP}°C\nElevated resource usage detected"
elif [ "$CPU" -le 5 ]; then
    TEXT="◆ STANDBY"
    CLASS="idle"
    TIP="CPU: ${CPU}% | MEM: ${MEM}% | TEMP: ${TEMP}°C\nFacility in low-power standby"
else
    TEXT="● NOMINAL"
    CLASS="nominal"
    TIP="CPU: ${CPU}% | MEM: ${MEM}% | TEMP: ${TEMP}°C\nAll systems operational"
fi

printf '{"text": "%s", "class": "%s", "tooltip": "%s"}\n' "$TEXT" "$CLASS" "$TIP"
