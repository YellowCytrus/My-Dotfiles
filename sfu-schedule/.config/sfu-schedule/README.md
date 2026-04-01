# SFU Schedule Viewer

Popup-панель для отображения учебного расписания СФУ с учётом чётности недели. Интегрируется с waybar как панель уведомлений (аналогично swaync).

## Особенности

- **Popup-панель** — безрамное окно поверх всех приложений
- **Позиционирование** — справа от экрана с отступом под waybar
- **Автозакрытие** — по клику вне окна или по Escape
- **Переключение недели** — кнопка для просмотра чётной/нечётной недели
- **Стили Tokyo Night** — гармонирует с waybar

## Сборка

```bash
cargo build --release
```

Бинарный файл: `target/release/sfu-schedule`

## Установка

```bash
# Сборка
cargo build --release

# Копирование бинарника
sudo cp target/release/sfu-schedule /usr/local/bin/

# Копирование конфига
mkdir -p ~/.config/sfu-schedule
cp schedule.json ~/.config/sfu-schedule/
```

## Конфигурация

Программа ищет `schedule.json` в:

1. `~/.config/sfu-schedule/schedule.json`
2. Рядом с исполняемым файлом
3. В текущей директории

### Формат schedule.json

```json
{
  "semester_start": "2024-09-02",
  "first_pair_time": "09:00",
  "pair_duration_minutes": 95,
  "break_duration_minutes": 10,
  "long_break_after_pair": 3,
  "long_break_duration_minutes": 35,
  "schedule": {
    "odd": {
      "monday": [
        { "pair": 1, "subject": "Математический анализ", "type": "лекция", "room": "А-101" }
      ],
      ...
    },
    "even": { ... }
  }
}
```

### Параметры

| Параметр | Описание |
|----------|----------|
| `semester_start` | Дата начала семестра (YYYY-MM-DD). Первая неделя — нечётная |
| `first_pair_time` | Время начала первой пары (HH:MM) |
| `pair_duration_minutes` | Продолжительность пары (95 мин) |
| `break_duration_minutes` | Короткий перерыв (10 мин) |
| `long_break_after_pair` | После какой пары большой перерыв (3) |
| `long_break_duration_minutes` | Продолжительность большого перерыва (35 мин) |

### Типы занятий

- `лекция` — синий (#7aa2f7)
- `практика` — зелёный (#98c379)
- `лаборатория` — оранжевый (#ff9e64)

## Интеграция с Waybar

### Конфигурация waybar

Добавьте в `~/.config/waybar/config`:

```json
{
  "modules-right": ["custom/schedule", ...],
  
  "custom/schedule": {
    "format": "󰈙",
    "tooltip": false,
    "on-click": "sfu-schedule"
  }
}
```

Или с toggle-скриптом:

```json
{
  "custom/schedule": {
    "format": "󰈙",
    "tooltip": false,
    "on-click": "~/.config/waybar/scripts/toggle-schedule.sh"
  }
}
```

### Скрипт toggle-schedule.sh

```bash
#!/bin/bash
if pgrep -x "sfu-schedule" > /dev/null; then
    pkill -x "sfu-schedule"
else
    sfu-schedule &
fi
```

```bash
chmod +x ~/.config/waybar/scripts/toggle-schedule.sh
```

### Стили для waybar (опционально)

```css
#custom-schedule {
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 8px;
    background: #3e4451;
    color: #c0caf5;
    font-size: 13px;
    min-width: 40px;
    opacity: 0.9;
}

#custom-schedule:hover {
    background: rgba(255, 255, 255, 0.12);
    opacity: 1.0;
}
```

## Управление

- **Escape** — закрыть панель
- **Клик вне панели** — закрыть панель
- **Кнопка "⇄ чёт/нечёт"** — переключить неделю

## Зависимости

- GTK4
- gtk4-layer-shell
- Rust 1.70+

На Arch Linux:

```bash
sudo pacman -S gtk4 gtk4-layer-shell
```

## Лицензия

MIT
