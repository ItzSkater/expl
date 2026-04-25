# expl

> btw i use arch

A universal Linux package manager for AppImage packages. Fast, simple, no root required.

**[Русский](#русский)**

---

## Why?

Most package managers are distro-specific. `expl` works on any Linux distro (except Ubuntu and its derivatives — just install Arch) and uses AppImage format so packages run anywhere without dependencies.

## Install

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

## Examples

```bash
expl -Ss cheat       # search for cheats
expl -S cheat-cs2    # install a package
expl -R cheat-cs2    # remove it
expl -Syu            # update everything
```

Packages are installed as AppImage files to `~/.local/bin/` and are ready to run immediately.

## Supported distros

Any Linux distro. Except:

- Ubuntu (and all its derivatives: Kubuntu, Xubuntu, Lubuntu, Ubuntu Budgie, Ubuntu Studio, Ubuntu Cinnamon, Ubuntu Unity, Ubuntu Kylin, Edubuntu)
- Linux Mint, Pop!_OS, Zorin OS, elementary OS, KDE Neon
- Peppermint OS, Vanilla OS, BackBox, Bodhi Linux, Linux Lite, Runtu, Voyage

Just install Arch :)

## Package repository

To add a package, submit a pull request with an entry in `index.json`:

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

Универсальный пакетный менеджер для Linux на основе AppImage. Быстрый, простой, не требует прав root.

### Зачем?

Большинство пакетных менеджеров привязаны к конкретному дистрибутиву. `expl` работает на любом Linux (кроме Ubuntu и производных — просто поставь Arch) и использует формат AppImage, поэтому пакеты запускаются везде без зависимостей.

### Установка

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

### Поддерживаемые дистрибутивы

Любой Linux. Кроме Ubuntu и всего что на ней основано. Просто поставь Arch :)

Пакеты устанавливаются как AppImage файлы в `~/.local/bin/` и сразу готовы к запуску.

### Лицензия

[GPL-3.0](LICENSE)
