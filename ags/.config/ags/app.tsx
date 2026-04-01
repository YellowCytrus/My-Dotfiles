#!/usr/bin/env -S ags run
import app from "ags/gtk3/app"
import { Astal, Gtk } from "ags/gtk3"
import { createPoll } from "ags/time"
import { exec, execAsync } from "ags/process"
import GLib from "gi://GLib"

const SRC = `${GLib.get_user_config_dir()}/ags`

function ApertureLogo() {
    return (
        <box class="aperture-logo">
            <label label="◉ APERTURE" />
        </box>
    )
}

function Workspaces() {
    const data = createPoll(
        { active: 1, occupied: [1] as number[] },
        300,
        () => {
            try {
                const active = JSON.parse(exec("hyprctl activeworkspace -j"))
                const wsList = JSON.parse(exec("hyprctl workspaces -j"))
                return {
                    active: active.id,
                    occupied: wsList
                        .filter((w: any) => w.id > 0)
                        .map((w: any) => w.id),
                }
            } catch {
                return { active: 1, occupied: [1] }
            }
        },
    )

    return (
        <box class="workspaces" spacing={2}>
            {Array.from({ length: 9 }, (_, i) => i + 1).map((id) => (
                <button
                    class={data((d) => {
                        const c = ["ws-btn"]
                        if (d.active === id) c.push("active")
                        else if (d.occupied.includes(id)) c.push("occupied")
                        return c.join(" ")
                    })}
                    onClicked={() =>
                        execAsync(`hyprctl dispatch workspace ${id}`)
                    }
                >
                    <label
                        label={id.toString().padStart(2, "0")}
                    />
                </button>
            ))}
        </box>
    )
}

function Clock() {
    const time = createPoll("", 1000, () => {
        const now = new Date()
        const h = now.getHours().toString().padStart(2, "0")
        const m = now.getMinutes().toString().padStart(2, "0")
        const s = now.getSeconds().toString().padStart(2, "0")
        return `${h}:${m}:${s}`
    })

    const date = createPoll("", 30000, () => {
        const now = new Date()
        const y = now.getFullYear()
        const mo = (now.getMonth() + 1).toString().padStart(2, "0")
        const d = now.getDate().toString().padStart(2, "0")
        return `${y}.${mo}.${d}`
    })

    return (
        <box class="clock" spacing={6}>
            <label class="clock-time" label={time} />
            <label class="clock-sep" label="//" />
            <label class="clock-date" label={date} />
        </box>
    )
}

function FacilityStatus() {
    return <label class="facility-status" label="FACILITY STATUS: NOMINAL" />
}

function CpuUsage() {
    const cpu = createPoll("--", 2000, () => {
        try {
            const out = exec([
                "bash",
                "-c",
                "top -bn1 | awk '/Cpu/{printf \"%d%%\", 100-$8}'",
            ])
            return out || "--"
        } catch {
            return "--"
        }
    })

    return (
        <box class="module" spacing={4}>
            <label class="module-icon" label="" />
            <label class="module-value" label={cpu} />
        </box>
    )
}

function MemUsage() {
    const mem = createPoll("--", 3000, () => {
        try {
            const out = exec([
                "bash",
                "-c",
                "free | awk '/Mem/{printf \"%d%%\", $3/$2*100}'",
            ])
            return out || "--"
        } catch {
            return "--"
        }
    })

    return (
        <box class="module" spacing={4}>
            <label class="module-icon" label="" />
            <label class="module-value" label={mem} />
        </box>
    )
}

function VolumeWidget() {
    const vol = createPoll("--", 1000, () => {
        try {
            const out = exec("wpctl get-volume @DEFAULT_AUDIO_SINK@")
            const match = out.match(/Volume:\s+(\d+\.?\d*)/)
            if (match) {
                const pct = Math.round(parseFloat(match[1]) * 100)
                const muted = out.includes("[MUTED]")
                return muted ? "MUTE" : `${pct}%`
            }
            return "--"
        } catch {
            return "--"
        }
    })

    const icon = createPoll("󰕾", 1000, () => {
        try {
            const out = exec("wpctl get-volume @DEFAULT_AUDIO_SINK@")
            if (out.includes("[MUTED]")) return "󰝟"
            const match = out.match(/Volume:\s+(\d+\.?\d*)/)
            if (match) {
                const pct = Math.round(parseFloat(match[1]) * 100)
                if (pct === 0) return "󰕿"
                if (pct < 50) return "󰖀"
                return "󰕾"
            }
            return "󰕾"
        } catch {
            return "󰕾"
        }
    })

    return (
        <box class="module" spacing={4}>
            <label class="module-icon" label={icon} />
            <label class="module-value" label={vol} />
        </box>
    )
}

function NetworkWidget() {
    const net = createPoll("--", 5000, () => {
        try {
            const out = exec([
                "bash",
                "-c",
                "nmcli -t -f TYPE,STATE,CONNECTION device | grep ':connected:' | head -1",
            ])
            const parts = out.split(":")
            if (parts.length >= 3 && parts[2]) {
                const name = parts[2]
                return name.length > 12 ? name.substring(0, 12) + ".." : name
            }
            return "OFFLINE"
        } catch {
            return "OFFLINE"
        }
    })

    const icon = createPoll("󰤭", 5000, () => {
        try {
            const out = exec([
                "bash",
                "-c",
                "nmcli -t -f TYPE,STATE device | grep ':connected' | head -1",
            ])
            if (out.includes("wifi")) return "󰤨"
            if (out.includes("ethernet")) return "󰈀"
            return "󰤭"
        } catch {
            return "󰤭"
        }
    })

    return (
        <box class="module" spacing={4}>
            <label class="module-icon" label={icon} />
            <label class="module-value" label={net} />
        </box>
    )
}

function BatteryWidget() {
    const bat = createPoll({ pct: "--", charging: false, level: 100 }, 10000, () => {
        try {
            const cap = exec("cat /sys/class/power_supply/BAT0/capacity").trim()
            const status = exec("cat /sys/class/power_supply/BAT0/status").trim()
            return {
                pct: `${cap}%`,
                charging: status === "Charging",
                level: parseInt(cap) || 100,
            }
        } catch {
            return { pct: "AC", charging: false, level: 100 }
        }
    })

    return (
        <box
            class={bat((b) => {
                const c = ["module", "battery"]
                if (b.level <= 10) c.push("critical")
                else if (b.level <= 20) c.push("low")
                if (b.charging) c.push("charging")
                return c.join(" ")
            })}
            spacing={4}
        >
            <label
                class="module-icon"
                label={bat((b) => {
                    if (b.charging) return "󰂄"
                    if (b.level <= 10) return "󰂃"
                    if (b.level <= 30) return "󰁼"
                    if (b.level <= 60) return "󰁾"
                    return "󰁹"
                })}
            />
            <label class="module-value" label={bat((b) => b.pct)} />
        </box>
    )
}

function Divider() {
    return <label class="divider" label="│" />
}

function Bar(monitor = 0) {
    return (
        <window
            class="bar"
            monitor={monitor}
            exclusivity={Astal.Exclusivity.EXCLUSIVE}
            anchor={
                Astal.WindowAnchor.TOP |
                Astal.WindowAnchor.LEFT |
                Astal.WindowAnchor.RIGHT
            }
        >
            <centerbox class="bar-inner">
                <box $type="start" class="bar-left" spacing={8}>
                    <ApertureLogo />
                    <Workspaces />
                </box>
                <box $type="center" class="bar-center" spacing={6}>
                    <Clock />
                    <FacilityStatus />
                </box>
                <box $type="end" class="bar-right" spacing={4}>
                    <CpuUsage />
                    <Divider />
                    <MemUsage />
                    <Divider />
                    <VolumeWidget />
                    <Divider />
                    <NetworkWidget />
                    <Divider />
                    <BatteryWidget />
                </box>
            </centerbox>
        </window>
    )
}

app.start({
    main() {
        app.apply_css(`${SRC}/style.css`)
        Bar(0)
    },
})
