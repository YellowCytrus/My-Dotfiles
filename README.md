# My-Dotfiles

Конфиги для Hyprland-стека в формате [GNU Stow](https://www.gnu.org/software/stow/): каждая папка верхнего уровня (`hypr`, `waybar`, …) — отдельный «пакет», внутри повторяется путь от `$HOME` (например `hypr/.config/hypr/`).

## Установка (Stow)

Из корня репозитория (не из `~`):

```bash
cd /path/to/My-Dotfiles
stow --target="$HOME" hypr waybar kitty rofi sfu-schedule fastfetch ags
# экран входа (тема SDDM в /usr/share — нужен root, только с --no-folding):
# sudo stow --no-folding --target=/ sddm
```

Поставить только часть пакетов — перечислите нужные имена.  
Если каталог уже существует и не пустой, Stow откажется; тогда либо временно переименуйте/уберите старую папку, либо используйте `stow --adopt` (осторожно: перезапишет ссылки).

Проверка без изменений:

```bash
stow -n --target="$HOME" hypr waybar
```

## Что внутри

| Пакет | Назначение |
|-------|------------|
| `hypr` | Hyprland, hypridle, hyprlock, скрипты |
| `waybar` | Панель (у вас исходники лежали в `~/dotfiles/config/waybar`, в репо зашиты как обычные файлы) |
| `kitty` | Терминал |
| `rofi` | Лаунчер и темы (симлинк `.current_wallpaper` не переносился) |
| `sfu-schedule` | Расписание SFU: код + `schedule.json` без `target/`, `.venv`, `*.7z` |
| `fastfetch` | Быстрый вывод инфо о системе |
| `ags` | Минимальная папка конфига AGS |
| `sddm` | Тема **`simple_sddm_2`** для SDDM (QML + ассеты + **видео-фон** `Cyberpunk_2077.mp4`) → **`sudo stow --no-folding --target=/ sddm`**. Подробности в [`sddm/README.md`](sddm/README.md). |

Исключено намеренно: виртуальные окружения, Rust `target/`, архивы `*.7z`, вложенный `.git` у расписания.

## Примечания

- Пути вида `/home/cytr/...` в конфигах при переносе на другую машину нужно поправить вручную.
- Для `sfu-schedule` после клонирования: свой `uv sync` / `cargo build` локально, если нужны бинарники.
