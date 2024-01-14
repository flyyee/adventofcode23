with open("testcase.txt", "r") as f:
    input = f.read(-1)

def is_symbol(char):
    if char == '*':
        return True
    return False
    

lines = input.split("\n")
height = len(lines)
width = len(lines[0])

part_areas = []

def add_part(i, j):
    to_add = []
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
            
            to_add.append((i__, j__))
    
    part_areas.append({"coverage": to_add, "count": 0, "values": []})

for i, line in enumerate(lines):
    for j, char in enumerate(line):
        if is_symbol(char):
            add_part(i, j)

# print(part_areas)

def look_for(i, j, number):
    for gear in part_areas:
        if (i, j) in gear["coverage"]:
            gear["count"] += 1
            gear["values"].append(number)
            return True

    return False

def handle_number(i, start, end, number):
    for j in range(start, end):
        if look_for(i, j, number):
            break

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
        
        try:
            number = int(line[start:end])
        except:
            pass
        handle_number(i, start, end, number)
        start = end
        end += 1

# print(part_areas)

ans = 0
for gear in part_areas:
    if gear["count"] == 2:
        ans += gear["values"][0] * gear["values"][1]

print(ans)