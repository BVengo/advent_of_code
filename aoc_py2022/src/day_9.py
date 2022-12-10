import numpy as np


with open('../input/day_9.txt', 'r') as f:
    lines = [tuple(l.split(' ')) for l in f.read().splitlines()]

directions = {
    'U': [0, -1],
    'D': [0, 1],
    'L': [-1, 0],
    'R': [1, 0]
}

def move_knotted_rope(n_knots: int):
    knot_positions = np.array([[0, 0]] * n_knots)

    visited = set()

    for head_direction, distance in lines:
        head_direction = directions[head_direction]
        distance = int(distance)

        for i in range(distance):
            knot_positions[0] += head_direction

            for j in range(1, len(knot_positions)):  # Update each tail knot accordingly
                dif = knot_positions[j-1] - knot_positions[j]

                if any(np.abs(dif) > 1):
                    knot_direction = np.clip(dif, -1, 1)
                    knot_positions[j] = knot_positions[j] + knot_direction

            visited.add(tuple(knot_positions[-1]))  # Track the tail

    return len(visited)


print(f'Part 1: {move_knotted_rope(2)}')
print(f'Part 2: {move_knotted_rope(10)}')