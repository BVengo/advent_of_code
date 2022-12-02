with open('../input/day_1.txt', 'r') as f:
    elves_list = ''.join(f.read()).split('\n\n')  # Split into a list of strings, with a blank line separator
    elves_list = [list(map(int, e.split('\n'))) for e in elves_list]  # Split strings by newline and cast to int

sums = sorted([sum(e) for e in elves_list], reverse=True)
print(sums[0])
print(sum(sums[0:3]))
