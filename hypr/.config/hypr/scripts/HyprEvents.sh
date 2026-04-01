#!/bin/bash
# Hyprland IPC event listener — plays sounds on workspace/window events

SOUND_DIR="$HOME/.local/share/sounds/portal2/stereo"

socat -U - UNIX-CONNECT:"$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock" 2>/dev/null | while read -r line; do
    case "$line" in
        workspace\>\>*)
            pw-play "$SOUND_DIR/workspace-switch.ogg" &
            ;;
    esac
done
