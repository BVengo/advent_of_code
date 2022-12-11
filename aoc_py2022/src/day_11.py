import copy
import re
import numpy as np


class Monkey:
    items: list[int]  # worry level
    operation: list[str, str, str]  # operation applied to old val
    test: int  # divisible by
    recipients: list[int, int]  # true, false
    num_inspected = 0

    def __init__(self, str_array: list[str]):
        self.items = [int(re.sub(',', '', x)) for x in str_array[1].split()[2:]]
        self.operation = str_array[2].split()[-3:]
        self.test = int(str_array[3].split()[-1])
        self.recipients = [int(str_array[i].split()[-1]) for i in range(4, 6)]

    def perform_inspection(self, item: int, relief_level: int):
        operation = [str(item) if val == 'old' else val for val in self.operation]
        return int(eval(''.join(operation)) / relief_level)

    def inspect_items(self, relief_level, all_monkeys, product_of_tests):
        for item in self.items:
            new_val = self.perform_inspection(item, relief_level) % product_of_tests
            all_monkeys[self.recipients[0 if (new_val % self.test == 0) else 1]].add_item(new_val)

        self.num_inspected += len(self.items)
        self.items = []  # should be holding no more items

    def add_item(self, new_item: int):
        self.items.append(new_item)


def run_phase(monkey_list, relief: int, rounds: int):
    [[m.inspect_items(relief, monkey_list, div_product) for m in monkey_list] for r in range(rounds)]
    sorted_monkeys = sorted(monkey_list, key=lambda m: m.num_inspected, reverse=True)
    return np.prod([m.num_inspected for m in sorted_monkeys[:2]])


with open('../input/day_11.txt', 'r') as f:
    inputs = [l.split("\n") for l in f.read().split("Monkey")[1:]]

monkeys = [Monkey(str_array) for str_array in inputs]
div_product = np.prod([m.test for m in monkeys])

print(f'Part 1: {run_phase(copy.deepcopy(monkeys), 3, 20)}')
print(f'Part 2: {run_phase(copy.deepcopy(monkeys), 1, 10000)}')
