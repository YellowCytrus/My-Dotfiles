# My-Dotfiles

Конфиги для Hyprland-стека в формате [GNU Stow](https://www.gnu.org/software/stow/): каждая папка верхнего уровня (`hypr`, `waybar`, …) — отдельный «пакет», внутри повторяется путь от `$HOME` (например `hypr/.config/hypr/`).

## Установка (Stow)

Из корня репозитория (не из `~`):

```bash
cd /path/to/My-Dotfiles

# Один заход: WM/терминал/лаунчер + весь «рисовой» окружение
stow --target="$HOME" \
  hypr waybar kitty rofi sfu-schedule fastfetch ags \
  fish swaync wlogout swappy btop cava matugen wallust \
  gtk3 gtk4 kvantum qt5ct qt6ct nwg-look mimeapps autostart systemd-user quickshell

# Экран входа (тема в /usr/share — нужен root, только с --no-folding):
# sudo stow --no-folding --target=/ sddm
```

Поставить только часть пакетов — перечислите нужные имена.  
Если каталог уже существует и не пустой, Stow откажется; тогда либо временно переименуйте/уберите старую папку, либо используйте `stow --adopt` (осторожно: перезапишет ссылки).

Проверка без изменений:

```bash
stow -n --target="$HOME" hypr fish gtk3
```

## Что внутри

| Пакет | Назначение |
|-------|------------|
| `hypr` | Hyprland, hypridle, hyprlock, скрипты |
| `waybar` | Панель |
| `kitty` | Терминал |
| `rofi` | Лаунчер и темы |
| `sfu-schedule` | Расписание SFU: код + `schedule.json` без `target/`, `.venv`, `*.7z` |
| `fastfetch` | Инфо о системе |
| `ags` | AGS |
| `fish` | Fish shell (`~/.config/fish`) |
| `swaync` | Уведомления (иконки, темы, `config.json`) |
| `wlogout` | Меню выхода |
| `swappy` | Скриншоты |
| `btop` | Монитор ресурсов |
| `cava` | Визуализатор аудио |
| `matugen` | Генерация тем из обоев |
| `wallust` | Цвета для тулчейна обоев |
| `gtk3` | `~/.config/gtk-3.0` |
| `gtk4` | `~/.config/gtk-4.0` |
| `kvantum` | `~/.config/Kvantum` |
| `qt5ct` / `qt6ct` | Внешний вид Qt |
| `nwg-look` | Настройки внешнего вида GTK |
| `mimeapps` | `~/.config/mimeapps.list` |
| `autostart` | `~/.config/autostart` |
| `systemd-user` | `~/.config/systemd/user` (в т.ч. `jarvis.service`, wants для PipeWire; см. битые симлинки ниже) |
| `quickshell` | 16× `*.qml` из корня `~/.config` (конфиг Quickshell) |
| `sddm` | Тема **`simple_sddm_2`** → **`sudo stow --no-folding --target=/ sddm`**. См. [`sddm/README.md`](sddm/README.md). |

При копировании из системы убран вложенный `.git` у `swaync`, чтобы не тянуть submodule.

## Примечания

- Пути вида `/home/cytr/...` в конфигах при переносе на другую машину нужно поправить вручную.
- Для `sfu-schedule` после клонирования: свой `uv sync` / `cargo build` локально, если нужны бинарники.
- **`systemd-user`:** если какой‑то `.service` в `*.wants` указывал на несуществующий юнит (не установлен пакет), симлинк может быть «битым» — это нормально до `systemctl --user daemon-reload` / установки соответствующего ПО.
