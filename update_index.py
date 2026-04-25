#!/usr/bin/env python3
"""
expl package index updater
Fetches AppImage releases from AppImageHub and GitHub
"""

import requests
import json
import re

GITHUB_API = "https://api.github.com/repos/{}/releases/latest"
APPIMAGE_HUB_FEED = "https://appimage.github.io/feed.json"
HEADERS = {"Accept": "application/vnd.github.v3+json"}

# Пакеты с GitHub (высокий приоритет, точные паттерны)
GITHUB_PACKAGES = {
    "obsidian": {
        "github": "obsidianmd/obsidian-releases",
        "asset_pattern": r"Obsidian-.*\.AppImage$",
        "description": "Markdown note-taking app"
    },
    "joplin": {
        "github": "laurent22/joplin",
        "asset_pattern": r"Joplin-.*\.AppImage$",
        "description": "Open source note-taking and to-do app"
    },
    "kdenlive": {
        "github": "KDE/kdenlive",
        "asset_pattern": r"kdenlive-.*\.AppImage$",
        "description": "Free and open source video editor"
    },
    "appflowy": {
        "github": "AppFlowy-IO/AppFlowy",
        "asset_pattern": r"AppFlowy-.*-linux-x86_64\.AppImage$",
        "description": "Open source Notion alternative"
    },
    "logseq": {
        "github": "logseq/logseq",
        "asset_pattern": r"Logseq-linux-x64-.*\.AppImage$",
        "description": "Privacy-first knowledge management"
    },
    "upscayl": {
        "github": "upscayl/upscayl",
        "asset_pattern": r"upscayl-.*-linux\.AppImage$",
        "description": "AI image upscaler"
    },
    "localsend": {
        "github": "localsend/localsend",
        "asset_pattern": r"LocalSend-.*-linux-x86-64\.AppImage$",
        "description": "AirDrop alternative for local network"
    },
    "zed": {
        "github": "zed-industries/zed",
        "asset_pattern": r"Zed.*\.AppImage$",
        "description": "High-performance code editor"
    },
    "gitbutler": {
        "github": "gitbutlerapp/gitbutler",
        "asset_pattern": r"GitButler.*\.AppImage$",
        "description": "Git client for modern workflows"
    },
    "heroic": {
        "github": "Heroic-Games-Launcher/HeroicGamesLauncher",
        "asset_pattern": r"Heroic-.*\.AppImage$",
        "description": "Epic and GOG games launcher"
    },
    "bottles": {
        "github": "bottlesdevs/Bottles",
        "asset_pattern": r"Bottles.*\.AppImage$",
        "description": "Run Windows software on Linux"
    },
    "bitwarden": {
        "github": "bitwarden/clients",
        "asset_pattern": r"Bitwarden-.*\.AppImage$",
        "description": "Open source password manager"
    },
    "flameshot": {
        "github": "flameshot-org/flameshot",
        "asset_pattern": r"Flameshot-.*\.AppImage$",
            "description": "Powerful screenshot tool"
    },
    "syncthing": {
        "github": "syncthing/syncthing",
        "asset_pattern": r"syncthing-linux-amd64-.*\.tar\.gz$",
        "description": "Continuous file synchronization"
    },
}


def get_github_release(repo: str, pattern: str) -> tuple[str, str] | None:
    url = GITHUB_API.format(repo)
    try:
        resp = requests.get(url, headers=HEADERS, timeout=10)
        resp.raise_for_status()
        data = resp.json()
    except Exception as e:
        print(f"  [!] Failed to fetch {repo}: {e}")
        return None

    version = data.get("tag_name", "unknown").lstrip("v")
    assets = data.get("assets", [])

    for asset in assets:
        name = asset.get("name", "")
        if re.search(pattern, name, re.IGNORECASE):
            return version, asset["browser_download_url"]

    print(f"  [!] No matching asset for {repo}")
    print(f"      Available: {[a['name'] for a in assets[:5]]}")
    return None


def fetch_appimage_hub() -> dict:
    print(":: Fetching AppImageHub catalog...")
    try:
        resp = requests.get(APPIMAGE_HUB_FEED, timeout=30)
        resp.raise_for_status()
        data = resp.json()
    except Exception as e:
        print(f"  [!] Failed to fetch AppImageHub: {e}")
        return {}

    packages = {}
    for app in data.get("items", []):
        name = app.get("name", "").lower().replace(" ", "-")
        if not name:
            continue

        description = app.get("description", "No description")
        links = app.get("links") or []
        categories = app.get("categories", [])

        # ищем прямую ссылку на AppImage
        url = None
        for link in links:
            href = link.get("url", "")
            if href.endswith(".AppImage"):
                url = href
                break

        if not url:
            continue

        packages[name] = {
            "version": "latest",
            "description": description,
            "url": url,
            "arch": ["x86_64"],
            "categories": categories
        }

    print(f"   Found {len(packages)} packages from AppImageHub")
    return packages


def main():
    print(":: Updating expl package index...\n")

    index = {}

    # Сначала AppImageHub
    hub_packages = fetch_appimage_hub()
    index.update(hub_packages)

    print()

    # Потом GitHub пакеты (перезаписывают AppImageHub если есть конфликт)
    for pkg_name, pkg_info in GITHUB_PACKAGES.items():
        print(f":: Checking {pkg_name}...")
        result = get_github_release(pkg_info["github"], pkg_info["asset_pattern"])

        if result:
            version, url = result
            index[pkg_name] = {
                "version": version,
                "description": pkg_info["description"],
                "url": url,
                "arch": ["x86_64"]
            }
            print(f"   v{version} -> {url[:60]}...")
        else:
            print(f"   skipped")

    with open("index.json", "w") as f:
        json.dump(index, f, indent=2)

    print(f"\n:: Done! {len(index)} packages in index.json")


if __name__ == "__main__":
    main()
