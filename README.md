# expl

> btw i use arch

A universal Linux package manager for AppImage packages. Fast, simple, no root required.

**[Русский](#русский) | [中文](#中文) | [Español](#español)**

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

---

## 中文

> 顺便说一句，我用 Arch

通用 Linux AppImage 包管理器。快速、简单、无需 root 权限。

### 为什么选择 expl？

大多数包管理器都依赖于特定发行版。`expl` 可在任何 Linux 发行版上运行（Ubuntu 及其衍生版除外——直接装 Arch 吧），使用 AppImage 格式，无需依赖即可运行。

### 安装

```bash
git clone https://github.com/ItzSkater/expl
cd expl
cargo build --release
sudo cp target/release/expl /usr/local/bin/
```

### 使用方法

```
expl -S  <包名>    安装包
expl -Ss <关键词>  搜索包
expl -R  <包名>    删除包
expl -Rns <包名>   删除包（含依赖）
expl -Syu          升级所有包
expl -Sy           同步包索引
expl -Sc           清理缓存
expl -Scc          完全清理缓存
expl -V            显示版本
```

### 支持的发行版

任何 Linux 发行版均可，Ubuntu 及其衍生版除外。直接装 Arch 吧 :)

包以 AppImage 文件形式安装到 `~/.local/bin/`，安装后即可直接运行。

---

## Español

> por cierto, uso Arch

Un gestor de paquetes universal para Linux basado en AppImage. Rápido, simple, sin necesidad de root.

### ¿Por qué?

La mayoría de los gestores de paquetes son específicos para cada distro. `expl` funciona en cualquier Linux (excepto Ubuntu y sus derivadas — simplemente instala Arch) y usa el formato AppImage para que los paquetes funcionen en cualquier lugar sin dependencias.

### Instalación

```bash
git clone https://github.com/ItzSkater/expl
cd expl
cargo build --release
sudo cp target/release/expl /usr/local/bin/
```

### Uso

```
expl -S  <paquete>  Instalar paquete
expl -Ss <consulta> Buscar paquete
expl -R  <paquete>  Eliminar paquete
expl -Rns <paquete> Eliminar paquete (con deps)
expl -Syu           Actualizar todos los paquetes
expl -Sy            Sincronizar índice
expl -Sc            Limpiar caché
expl -Scc           Limpiar caché completo
expl -V             Mostrar versión
```

### Distros soportadas

Cualquier distro Linux. Excepto Ubuntu y todo lo basado en ella. Simplemente instala Arch :)

Los paquetes se instalan como archivos AppImage en `~/.local/bin/` y están listos para ejecutarse inmediatamente.
