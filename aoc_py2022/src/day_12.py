with open("../input/day_12.txt", 'r') as f:
    input_text = f.read().split('\n')

# Complex numbers to easily query adjacent cells
grid = {x + y * 1j: e for y, l in enumerate(input_text)
        for x, e in enumerate(l)}

start_position = [k for k, v in grid.items() if v == 'S'][0]
end_position = [k for k, v in grid.items() if v == 'E'][0]

# Set start/end vals to correct elevation
grid[start_position] = 'a'
grid[end_position] = 'z'

shortest_paths = {end_position: 0}
stack = [end_position]

while len(stack) > 0:
    current_node = stack.pop(0)
    adjacent_nodes = [current_node + direction for direction in [1, -1, 1j, -1j]]
    for node in adjacent_nodes:
        if not node in shortest_paths and node in grid and ord(grid[current_node]) - ord(grid[node]) <= 1:
            shortest_paths[node] = shortest_paths[current_node] + 1
            stack.append(node)

print(shortest_paths[start_position])
print(sorted(shortest_paths[p] for p in shortest_paths if grid[p] in "Sa")[0])