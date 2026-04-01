#!/bin/bash

get() {
    hyprctl getoption "$1" | awk '/custom/ {print "#"substr($3,1,6)}'
}

ACTIVE=$(get general:col.active_border)
INACTIVE=$(get general:col.inactive_border)
BG="#1a1b26" # fallback Tokyo Night

cat <<EOF
:root {
    --accent: ${ACTIVE};
    --accent-inactive: ${INACTIVE};
    --bg: ${BG};
}
EOF
