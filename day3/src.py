with open("testcase.txt", "r") as f:
    input = f.read(-1)

def is_symbol(char):
    if not char.isnumeric() and char != '.':
        return True
    return False
    

lines = input.split("\n")
height = len(lines)
width = len(lines[0])

part_areas = []

def add_part(i, j):
    for i_ in range(-1, 2):
        for j_ in range(-1, 2):
            if (i_, j_) == (0, 0):
                continue
            i__ = i + i_
            j__ = j + j_
            if i__ < 0 or i__ >= height:
                continue
            if j__ < 0 or j__ >= width:
                continue
            
            part_areas.append((i__, j__))

for i, line in enumerate(lines):
    for j, char in enumerate(line):
        if is_symbol(char):
            add_part(i, j)

# print(part_areas)

def handle_number(i, start, end):
    for j in range(start, end):
        if (i, j) in part_areas:
            return True

    return False

parts = []

for i, line in enumerate(lines):
    start = 0
    end = 1
    while end <= width:
        while not line[start:end].isnumeric() and not end > width:
            start += 1
            end += 1
        while line[start:end].isnumeric() and not end > width:
            end += 1
        if not line[start:end].isnumeric():
            end -= 1
        if handle_number(i, start, end):
            parts.append(int(line[start:end]))
        start = end
        end += 1

print(parts)

print(sum(parts))