import requests

page = requests.get("https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_version_numbers")


from bs4 import BeautifulSoup;

soup = BeautifulSoup(page.content, "html.parser")

tbody = list(map(lambda t: str(t), soup
    .find(id = "Versions_after_the_Netty_rewrite")
    .parent.parent.parent
    .find("table", class_ = "wikitable")
    .find("tbody")
    .children
))
tbody.pop(0)
tbody = "".join(tbody)


from html.parser import HTMLParser

col = 0
cell = ""
row = []
snapshot = None
id = None
by_id = {}
class MyHTMLParser(HTMLParser):

    def handle_starttag(self, tag, attrs):
        global col, snapshot
        if (tag == "tr"):
            col = 0
            snapshot = None
        elif (tag == "td"):
            col += 1
        elif (tag == "abbr"):
            for attr in attrs:
                if (attr[0] == "title"):
                    attr1 = str(attr[1])
                    if (attr1.startswith("Snapshot;")):
                        snapshot = attr1.split(" ")[-1]

    def handle_endtag(self, tag):
        global cell, row, snapshot, id, by_id
        if (tag == "tr"):
            go = True
            if (len(row) == 3):
                try:
                    id = int(row[1])
                except ValueError:
                    if (snapshot == None):
                        go = False
                    else:
                        id = int(snapshot)
            if (go):
                if (id in by_id):
                    by_id[id].append(row[0])
                else:
                    by_id[id] = [row[0]]
            row = []
        elif (tag == "td"):
            row.append(cell)
            cell = ""

    def handle_data(self, data):
        global cell
        cell += data.rstrip("\n")

parser = MyHTMLParser()
parser.feed(str(tbody))


with open("out.rs", "w") as f:
    f.write("impl Protocol {\n")
    f.write(f"    const LOOKUP : [(u32, &'static [&'static str],); {len(by_id)}] = [\n")
    for id in by_id.keys():
        names = by_id[id]
        names.reverse()
        names = ", ".join(map(lambda name: f"\"{name}\"", names))
        f.write(f"        ({id}, &[{names}],),\n")
    f.write("    ];\n")
    f.write("}\n")
