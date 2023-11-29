from dataclasses import dataclass, field
from typing import List, Self

TEST_DATA = """$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k"""


@dataclass
class FSEntry:
    name: str
    size: int = 0
    children: List[Self] = field(default_factory=list)
    parent: Self = None

    @property
    def is_dir(self):
        return self.size == 0

    def total_size(self, filter_func=None) -> int:
        if not filter_func:
            filter_func = lambda size: True
        total = self.size
        for child in self.children:
            total += child.total_size(filter_func)

        if filter_func(total):
            return total
        else:
            return 0

    def get_child(self, path) -> Self:
        for child in self.children:
            if child.name == path:
                return child
        return None

    def add(self, child):
        child.parent = self
        self.children.append(child)

    def __repr__(self):
        return self.fully_qualified_name

    @property
    def fully_qualified_name(self) -> str:
        if self.parent:
            parent = self.parent.fully_qualified_name
            if parent == "/":
                return f"/{self.name}"
            else:
                return f"{parent}/{self.name}"

        return self.name

    def print_tree(self, indent=0):
        line = " " * indent
        line += f"- {self.name} ("
        if self.is_dir:
            line += f"dir, total_size={self.total_size():,}"
            if self.total_size() <= 100_000:
                line += ", MATCHES"
            line += ")"
        else:
            line += f"file, size={self.size:,})"

        print(line.rjust(indent + len(line)))

        for child in self.children:
            child.print_tree(indent + 1)


def get_from_fs(fs, path):
    current = fs
    if path == [""]:
        return fs

    for entry in path[1:]:
        current = current.get_child(entry)

    return current


def main(data):
    root_fs = FSEntry(name="/")
    cwd = [""]
    is_listing = False

    for line in data.split("\n"):
        line = line.strip()
        print(line)

        # New Implementation
        if line.startswith("$"):
            is_listing = False
        elif is_listing:
            size, filename = line.split(" ")
            if not get_from_fs(root_fs, [*cwd, filename]):
                parent = get_from_fs(root_fs, cwd)
                thing_type = "dir" if size == "dir" else "file"
                print(f"Creating {thing_type} at {parent}/{filename}")
                parent.add(
                    FSEntry(
                        name=filename,
                        size=0 if size == "dir" else int(size)
                    )
                )

        if line == "$ ls":
            is_listing = True
        elif line.startswith("$ cd "):
            new_dir = line.split("cd ", 1)[1]
            if new_dir == "/":
                cwd = [""]
            elif new_dir == "..":
                cwd.pop()
            else:
                if not get_from_fs(root_fs, [*cwd, new_dir]):
                    parent = get_from_fs(root_fs, cwd)
                    parent.add(FSEntry(name=new_dir))

                cwd.append(new_dir)
            print(f"Changed to {cwd}")

    root_fs.print_tree()

    size_filter = lambda size: size <= 100_000

    def child_sizes(parent) -> int:
        total = 0
        for child in parent.children:
            if child.is_dir:
                total += child_sizes(child)

            if child.total_size(size_filter) and child.is_dir:
                total += child.total_size()
        return total

    print(f"Part 1: {child_sizes(root_fs)}")


def banner(banner):
    print(f"{banner:=^40}")


if __name__ == "__main__":
    # banner("Demo Data")
    # main(TEST_DATA)
    banner("Live Data")
    with open("data/day7.txt") as df:
        main(df.read())
