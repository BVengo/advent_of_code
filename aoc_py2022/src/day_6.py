def marker_start(signal:str, n: int):
    char_list = [None] * 26  # idx: char, val: last seen index
    count = 0
    for i, char in enumerate(signal[:-(n-1)]):
        int_val = ord(char) - 97
        count += 1
        if char_list[int_val] is not None and char_list[int_val] > i - count:
            count = (i - char_list[int_val])
        char_list[int_val] = i

        if count == n:
            break

    return i+1


with open('../input/day_6.txt', 'r') as f:
    input = f.read()

print(marker_start(input, 4))   # Part 1
print(marker_start(input, 14))  # Part 2
