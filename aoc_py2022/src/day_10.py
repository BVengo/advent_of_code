import numpy as np


with open('../input/day_10.txt') as f:
    lines = f.read().splitlines()

# Fill all values
strengths = np.array([[1, 1]])  # [cycle, strength]
for i, l in enumerate(lines):
    prev = strengths[i]

    if l == 'noop':
        strengths = np.append(strengths, [prev + [1, 0]], axis=0)
    else:
        _, val = tuple(l.split(' '))
        strengths = np.append(strengths, [prev + [2, int(val)]], axis=0)

# Part 1:
part_1_sum = 0
for query in [20, 60, 100, 140, 180, 220]:
    for i, (cycle, strength) in enumerate(strengths):
        if cycle < query:
            continue

        if cycle == query:
            part_1_sum += (query * strength)
        elif cycle > query:  # Overshot, grab previous
            part_1_sum += (query * strengths[i-1][1])

        break

print(f'Part 1: {part_1_sum}')


# Part 2:
def update_crt(c, r) -> str:
    return '#' if r - 1 <= c <= r + 1 else '.'


width = 40

crt = ''
cycle = 0
register = 1

for l in lines:
    rep = 1
    val = 0

    if l != 'noop':
        rep = 2
        _, val = tuple(l.split(' '))

    for i in range(rep):
        crt += update_crt(cycle % width, register)
        cycle += 1

    register += int(val)

print('\n'.join([crt[i:i+width] for i in range(0, len(crt), width)]))