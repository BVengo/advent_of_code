import numpy as np
import copy

with open('../input/day_5.txt', 'r') as f:
    lines = f.read().splitlines()

n_stacks = int((len(lines[0])+1)/4)

stack_inputs = list(filter(lambda l: '[' in l, lines))

stacks = np.array([[s[(i*4)+1] for i in range(n_stacks)] for s in stack_inputs]).T
stacks = [s[s!=' '].tolist() for s in stacks]
[s.reverse() for s in stacks]

# move [0] from [1] to [2]
instructions = list(filter(lambda l: l.startswith('move'), lines))
instructions = [l.split(' ') for l in instructions]
instructions = [(int(l[1]), int(l[3]), int(l[5])) for l in instructions]

# Stage 1
s1_stacks = copy.deepcopy(stacks)

for instruction in instructions:
    for i in range(instruction[0]):
        if len(s1_stacks[instruction[1]-1]) == 0:
            break

        s1_stacks[instruction[2]-1].append(s1_stacks[instruction[1]-1].pop())

print(''.join([s[-1] for s in s1_stacks]))

# Stage 2
s2_stacks = copy.deepcopy(stacks)

for instruction in instructions:
    quantity = instruction[0]
    stack_from = s2_stacks[instruction[1]-1]
    stack_to = s2_stacks[instruction[2]-1]

    if len(stack_from) < quantity:
        quantity = len(stack_from)

    stack_to.extend(stack_from[-quantity:])
    del stack_from[-quantity:]

print(''.join([s[-1] for s in s2_stacks]))
