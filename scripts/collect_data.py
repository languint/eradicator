import requests
from bs4 import BeautifulSoup
import re

BASE = "https://tdx.fandom.com"
TOWERS_LIST = "https://tdx.fandom.com/wiki/Towers"

FILE_PATH = "../library/eradicator_core/src/towers.rs"

HEADERS = {"User-Agent": "Mozilla/5.0"}

def fetch(url):
    r = requests.get(url, headers=HEADERS)
    r.raise_for_status()
    return BeautifulSoup(r.text, "html.parser")

def get_all_tower_pages():
    soup = fetch(TOWERS_LIST)
    links = []
    for t in soup.select(".i__icon__text"):
        a = t.find('a')
        href = a.get("href")
        print(href)
        if href and "/wiki/" in href:
            links.append(BASE + href)

    return list(dict.fromkeys(links))

money_regex = re.compile(r"\$([\d,]+)")

def scrape_upgrades(url):
    soup = fetch(url)

    upgrade_header = None
    for h in soup.select("h2, h3"):
        if "Upgrade" in h.get_text():
            upgrade_header = h
            break

    if upgrade_header is None:
        return None, None

    costs = []
    node = upgrade_header.find_next_sibling()
    while node and node.name not in ("h2", "h3"):
        text = node.get_text(" ", strip=True)
        for m in money_regex.findall(text):
            costs.append(int(m.replace(",", "")))
        node = node.find_next_sibling()

    if len(costs) >= 10:
        return costs[:5], costs[5:10]

    return None, None


def rust_ident(name: str) -> str:
    """Convert a tower name like 'Cryo Ranger' → 'CryoRanger'."""
    cleaned = re.sub(r"[^a-zA-Z0-9 ]", "", name)
    parts = cleaned.split()
    return "".join(p.capitalize() for p in parts)


def main():
    pages = get_all_tower_pages()

    towers = []
    upgrades = []

    for url in pages:
        name = url.split("/")[-1].replace("_", " ")
        rust_name = rust_ident(name)

        print(f"Scraping {name} ...")

        top, bottom = scrape_upgrades(url)
        towers.append(rust_name)
        upgrades.append((top, bottom))

    with open(FILE_PATH, "w+") as f:
        f.write("use crate::defs::NrOf;\n")
        f.write("\n")
        f.write("pub enum Towers {\n")
        for t in towers:
            f.write(f"    {t},\n")
        f.write("}\n")
        f.write("\n#[rustfmt::skip]\n")
        f.write("pub const TOWER_UPGRADES: [[Option<[u64; 5]>; 2]; NrOf::TOWERS] = [\n")

        for (top, bottom) in upgrades:
            def fmt(path):
                if path is None:
                    return "None"
                nums = ", ".join(str(x) for x in path)
                return f"Some([{nums}])"

            f.write(f"    [{fmt(top)}, {fmt(bottom)}],\n")

        f.write("];\n")


if __name__ == "__main__":
    main()
