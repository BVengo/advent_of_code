from __future__ import annotations


class Node:
    """ A node in a linked file system """
    name: str
    size: int
    parent: Node | None
    children: dict[str, Node]

    def __init__(self, name: str, size: int, parent: Node | None):
        self.name = name
        self.size = size
        self.parent = parent
        self.children = {}

    def add_child(self, child: Node):
        child.parent = self
        self.children[child.name] = child
        self.size += child.size

    def update_size(self):
        self.size = sum([c.size for c in self.children.values()])

    def move(self, direction: str):
        if direction == '.':
            return self
        if direction == '..':
            if self.parent is None:
                raise Exception("Parent does not exist!")
            self.parent.update_size()
            return self.parent

        if direction in self.children.keys():
            return self.children[direction]

        raise Exception(f"That resource is not a valid path! Provided value: {direction}")

    def flatten(self):
        flat_list = [self]
        [flat_list.extend(c.flatten()) for c in self.children.values()]
        return flat_list


# Populate directories
with open('../input/day_7.txt', 'r') as f:
    commands = f.read().splitlines()

cur = Node("/", 0, None)
for output in commands[1:]:
    command = output[:4]
    match command:
        case '$ ls': # Listing files, so just continue
            pass
        case '$ cd': # Go to child
            cur = cur.move(output[5:])
        case _:
            size, name = output.split(' ')
            cur.add_child(Node(name, int(size) if size != 'dir' else 0, None))

while cur.parent is not None:
    cur = cur.move('..')

# Part 1
flattened_list = cur.flatten()
filtered_list = list(filter(lambda n: len(n.children) > 0 and n.size <= 100000, flattened_list))

print(f'Part 1: {sum([n.size for n in filtered_list])}')

# Part 2
needed_space = 30000000 - (70000000 - cur.size)
filtered_list = list(filter(lambda n: len(n.children) > 0 and n.size >= needed_space, flattened_list))
print(f'Part 2: {min([n.size for n in filtered_list])}')