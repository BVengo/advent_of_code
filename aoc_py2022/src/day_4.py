import re

with open('../input/day_4.txt', 'r') as f:
    pairs = [re.split(',|-', l) for l in f.read().splitlines()]
    pairs = [(range(int(p[0]), int(p[1])+1), range(int(p[2]), int(p[3])+1)) for p in pairs]

# Part 1
total_overlaps = [all(r in p[0] for r in p[1]) or all(r in p[1] for r in p[0]) for p in pairs]
print(sum(total_overlaps))

# Part 2
partial_overlaps = [any(r in p[0] for r in p[1]) for p in pairs]
print(sum(partial_overlaps))
