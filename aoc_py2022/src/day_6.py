def marker_start(signal:str, n: int):
    char_dict = {}  # char: last_index
    count = 0
    for i, char in enumerate(signal[:-(n-1)]):
        count += 1
        if char in char_dict and char_dict[char] > i - count:
            count = (i - char_dict[char])
        char_dict[char] = i

        if count == n:
            break

    return i+1


with open('../input/day_6.txt', 'r') as f:
    input = f.read()

print(marker_start(input, 4))   # Part 1
print(marker_start(input, 14))  # Part 2
