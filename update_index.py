#!/usr/bin/env python3
"""
expl package index updater
Fetches latest AppImage releases from GitHub and updates index.json
"""

import requests
import json
import re
import sys

PACKAGES = {
    "bambu-studio": {
        "github": "bambulab/BambuStudio",
        "asset_pattern": r"Bambu_Studio_linux_ubuntu.*\.AppImage$",
        "description": "3D printer slicer for Bambu Lab printers"
    },
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
}

GITHUB_API = "https://api.github.com/repos/{}/releases/latest"
HEADERS = {"Accept": "application/vnd.github.v3+json"}


def get_latest_release(repo: str, pattern: str) -> tuple[str, str] | None:
    """Returns (version, download_url) or None"""
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

    print(f"  [!] No matching asset for {repo} (pattern: {pattern})")
    print(f"      Available: {[a['name'] for a in assets[:5]]}")
    return None


def main():
    print(":: Updating expl package index...\n")

    index = {}

    for pkg_name, pkg_info in PACKAGES.items():
        print(f":: Checking {pkg_name}...")
        result = get_latest_release(pkg_info["github"], pkg_info["asset_pattern"])

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
