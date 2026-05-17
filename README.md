# expl

> btw i use arch

A smart package manager for Arch-based Linux. Installs AppImage packages from its own index, falls back to `yay` or `pacman` for everything else. Fast, simple, no root required.

**[Русский](#русский)**

---

## Why?

`expl` is built for Arch-based distros. It first searches its own AppImage index, and if a package isn't there — automatically falls back to `yay` or `pacman`.

## Install

```bash
yay -S expl --noconfirm
```

Or build from source:

```bash
git clone https://github.com/ItzSkater/expl
cd expl
cargo build --release
sudo cp target/release/expl /usr/local/bin/
```

## Usage

```
expl -S  <pkg>     Install package
expl -Ss <query>   Search for package
expl -R  <pkg>     Remove package
expl -Rns <pkg>    Remove package (with deps)
expl -Syu          Upgrade all packages
expl -Sy           Sync package index
expl -Sc           Clean cache
expl -Scc          Full cache clean
expl -V            Print version
```

## How it works

1. Searches own AppImage index
2. If not found — syncs index and tries again
3. If still not found — falls back to `yay` or `pacman`

## Examples

```bash
expl -Ss firefox     # search
expl -S firefox      # install (yay fallback if not in index)
expl -S obsidian     # install from AppImage index
expl -Syu            # update everything
```

## Supported distros

Arch-based only. Tested on Arch Linux and EndeavourOS.

Not for Ubuntu, Debian, Fedora, or anything else. Just install Arch :)

## Add a package to index

Submit a pull request with an entry in `index.json`:

```json
{
  "your-package": {
    "version": "1.0.0",
    "description": "Short description",
    "url": "https://github.com/you/repo/releases/download/v1.0.0/package.AppImage",
    "arch": ["x86_64"]
  }
}
```

## Built with

- [Rust](https://www.rust-lang.org/)
- [tokio](https://tokio.rs/) — async runtime
- [reqwest](https://github.com/seanmonstar/reqwest) — HTTP client
- [indicatif](https://github.com/console-rs/indicatif) — progress bars

## License

[GPL-3.0](LICENSE)

---

## Русский

> кстати, я использую arch

Умный пакетный менеджер для Arch-based Linux. Устанавливает AppImage пакеты из собственного индекса, при отсутствии — автоматически использует `yay` или `pacman`. Быстрый, простой, не требует прав root.

### Зачем?

`expl` создан для Arch-based дистрибутивов. Сначала ищет в собственном AppImage индексе, если не нашёл — фолбэк на `yay` или `pacman`.

### Установка

```bash
yay -S expl --noconfirm
```

Или из исходников:

```bash
git clone https://github.com/ItzSkater/expl
cd expl
cargo build --release
sudo cp target/release/expl /usr/local/bin/
```

### Использование

```
expl -S  <пакет>   Установить пакет
expl -Ss <запрос>  Поиск пакета
expl -R  <пакет>   Удалить пакет
expl -Rns <пакет>  Удалить пакет (с зависимостями)
expl -Syu          Обновить все пакеты
expl -Sy           Синхронизировать индекс
expl -Sc           Очистить кэш
expl -Scc          Полная очистка кэша
expl -V            Версия
```

### Как работает

1. Ищет в собственном AppImage индексе
2. Если не нашёл — синхронизирует индекс и пробует снова
3. Если всё равно не нашёл — фолбэк на `yay` или `pacman`

### Поддерживаемые дистрибутивы

Только Arch-based. Протестировано на Arch Linux и EndeavourOS.

Не для Ubuntu, Debian, Fedora и прочего. Просто поставь Arch :)

### Лицензия

[GPL-3.0](LICENSE)
