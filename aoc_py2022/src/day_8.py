import numpy as np


tree_map = np.genfromtxt('../input/day_8.txt', delimiter=1, dtype=int)
view_map = np.empty(tree_map.shape, dtype=object)

for (y, x), val in np.ndenumerate(tree_map):
    # Part 1
    max_left = max(tree_map[y, x - 1], view_map[y, x - 1][1][0]) if x > 0 else -1
    max_up = max(tree_map[y - 1, x], view_map[y - 1, x][1][1]) if y > 0 else -1
    max_right = np.max(tree_map[y, x + 1:]) if x < tree_map.shape[1] - 1 else -1
    max_down = np.max(tree_map[y + 1:, x]) if y < tree_map.shape[0] - 1 else -1
    comparisons = (max_left, max_up, max_right, max_down)
    success = any(val > comparisons)

    # Part 2
    score_left = next((idx + 1 for idx, val2 in enumerate(tree_map[y, :x][::-1]) if val2 >= val), x) if x > 0 else 0
    score_up = next((idx + 1 for idx, val2 in enumerate(tree_map[:y, x][::-1]) if val2 >= val), y) if y > 0 else 0
    score_right = next((idx + 1 for idx, val2 in enumerate(tree_map[y, x + 1:]) if val2 >= val), tree_map.shape[1] - x - 1) if x < tree_map.shape[1] - 1 else 0
    score_down = next((idx + 1 for idx, val2 in enumerate(tree_map[y + 1:, x]) if val2 >= val), tree_map.shape[0] - y - 1) if y < tree_map.shape[0] - 1 else 0

    score = score_left * score_up * score_right * score_down
    view_map[y, x] = (success, comparisons[:2], score)

print(f'Part 1: {np.sum([success for idx, (success, *_) in np.ndenumerate(view_map)])}')
print(f'Part 2: {np.max([score for idx, (*_, score) in np.ndenumerate(view_map)])}')
