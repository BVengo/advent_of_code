with open('../input/day_3.txt', 'r') as f:
    sacks = f.read().splitlines()

# Part 1
indices = [int(len(s)/2) for s in sacks]
partitioned_sacks = [(sacks[i][:indices[i]], sacks[i][indices[i]:]) for i in range(0, len(sacks))]
overlaps = [''.join(set(s[0]).intersection(s[1])) for s in partitioned_sacks]
scores = [ord(o) - 38 if o.isupper() else ord(o) - 96 for o in overlaps]
print(sum(scores))

# Part 2
group_size = 3
groups = [sacks[i:i+group_size] for i in range(0, len(sacks), group_size)]
overlaps = [''.join(set(g[0]).intersection(g[1]).intersection(g[2])) for g in groups]
scores = [ord(o) - 38 if o.isupper() else ord(o) - 96 for o in overlaps]
print(sum(scores))