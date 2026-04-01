# SDDM: simple_sddm_2 с видео-фоном

Здесь лежит снимок темы **`simple_sddm_2`** из `/usr/share/sddm/themes/`, как у тебя сейчас: в `theme.conf` фон — **`Backgrounds/Cyberpunk_2077.mp4`** (живое видео).

## Установка через Stow (нужен root, цель — `/`)

Из корня репозитория **My-Dotfiles**:

```bash
cd /path/to/My-Dotfiles
# Важно: --no-folding обязателен. Иначе Stow свернёт дерево в один symlink на …/usr
# и при --target=/ подменит весь /usr (опасно для системы).
sudo stow --no-folding -v --target=/ sddm
```

Если каталог `/usr/share/sddm/themes/simple_sddm_2` уже существует и это не symlink от Stow, команда откажется: временно переименуй старую папку или сделай бэкап, затем снова `sudo stow --no-folding --target=/ sddm`.

Проверка «сухой прогон»:

```bash
sudo stow -n -v --no-folding --target=/ sddm
```

## Настройка SDDM

Убедись, что в **`/etc/sddm.conf`** указана эта тема (пример полного файла см. `sddm.conf.example` в этой папке):

```ini
[Theme]
Current=simple_sddm_2
```

После правок:

```bash
sudo systemctl restart sddm
```

(лучше проверять из TTY, чтобы не потерять сессию неожиданно.)

## Другой монитор / фон

Разрешение задаётся в `theme.conf` темы (`ScreenWidth` / `ScreenHeight`). Чтобы сменить ролик или картинку — положи файл в `.../simple_sddm_2/Backgrounds/` и поправь строку `Background=` **относительным** путём (как сейчас `Backgrounds/Cyberpunk_2077.mp4`).
