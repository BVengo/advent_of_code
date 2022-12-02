with open('../input/day_2.txt', 'r') as f:
    rounds = [l.split(' ') for l in f.read().splitlines()]  # read in
    rounds = [(ord(x) - 64, ord(y) - 87) for x, y in rounds]  # convert to int

part_1 = [(y + (((y + 1) % 3) + 1 == x) * 6 + (y == x) * 3) for x, y in rounds]
part_2 = [((y == 1) * (((x + 1) % 3) + 1) + (y == 2) * (x + 3) + (y == 3) * ((x % 3) + 7)) for x, y in rounds]

print(sum(part_1))
print(sum(part_2))
