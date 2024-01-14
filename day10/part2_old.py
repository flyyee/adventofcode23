with open("example2.txt", "r") as f:
    input = f.read(-1)

grid = input.split("\n")

for i, row in enumerate(grid):
    for j, char in enumerate(row):
        if char == "S":
            curr = (i, j)
            break
    else:
        continue
    break

def is_valid(pair):
    i, j = pair
    if i < 0 or i >= len(grid) or j < 0 or j >= len(grid[0]):
        return False
    return True

char_to_directions = {
    "|": ("U", "D"), 
    "-":("L", "R"), 
    "L":("U", "R"), 
    "J":("U", "L"), 
    "7":("D", "L"), 
    "F":("D", "R"),
    ".": None
}

oppo = {
    "U": "D",
    "D": "U",
    "L": "R",
    "R": "L",
    None: None
}

direction_to_offset = {
    "R": (0, 1),
    "U": (-1, 0),
    "L": (0, -1),
    "D": (1, 0)
}

# print(curr)
adjacents = [((0, 1), "R"), ((1, 0), "D"), ((0, -1), "L"), ((-1, 0), "U")]
possible_starts = []
for adjacent, my_direction in adjacents:
    i, j = curr[0] + adjacent[0], curr[1] + adjacent[1]
    if is_valid([i, j]):
        their_directions = char_to_directions[grid[i][j]]
        if their_directions == None:
            continue

        for their_direction in their_directions:
            if their_direction == oppo[my_direction]:
                possible_starts.append([[i, j], my_direction])
                break
    

# print(possible_starts)
found = False
for (i, j), my_direction in possible_starts:
    path = []
    while True:
        path.append((i, j))

        if grid[i][j] == "S":
            found = True
            break
        
        their_directions = char_to_directions[grid[i][j]]
        if their_directions == None:
            break
        
        accessible = False
        for possible_direction in their_directions:
            if possible_direction == oppo[my_direction]:
                accessible = True
            else:
                new_direction = possible_direction

        if not accessible:
            break

        my_direction = new_direction
        i += direction_to_offset[my_direction][0]
        j += direction_to_offset[my_direction][1]

    if found:
        break
    
print(path)
# vertices = []
# for i, j in path:
#     if grid[i][j] in ["L", "J", "F", "7", "S"]:
#         vertices.append([i, j])

# print(vertices)

# def regularise(vertices):
#     xs = [v[0] for v in vertices]
#     xs.sort()
#     xs = set(xs)
#     xsm = {}
#     for rank, val in enumerate(xs):
#         xsm[val] = rank
#     for v in vertices:
#         rank = xsm[v[0]]
#         v[0] += rank

#     ys = [v[1] for v in vertices]
#     ys.sort()
#     ys = set(ys)
#     ysm = {}
#     for rank, val in enumerate(ys):
#         ysm[val] = rank
#     print(ysm)

#     for v in vertices:
#         rank = ysm[v[1]]
#         v[1] += rank

# regularise(vertices)
# print(vertices)

# def shoelace(vertices):
#     area = 0
#     n = len(vertices)
#     for i in range(1, n + 1):
#         area += vertices[i % n][1] * (vertices[(i - 1) % n][0] - vertices[(i + 1) % n][0])
#     return abs(area / 2)

# print(shoelace(vertices))
# area = shoelace(vertices) - len(path)
# print(area)

# quit()

# ROW-WISE
rows = {}
for x, (i, j) in enumerate(path):
    if i not in rows:
        rows[i] = []
    if x > 0 and path[x - 1][0] == i:
        # same row
        rows[i][-1].append(j)
    else:
        # diff row
        rows[i].append([j])

if path[0][0] == path[-1][0]:
    rows[i][0] += rows[i][-1]
    rows[i][0].sort()
    rows[i] = rows[i][:-1]

for key in rows:
    for i in rows[key]:
        i.sort()
    rows[key].sort()
print(rows)

row_oks = []
for key in rows:
    outside = True
    rows[key].sort()
    row = rows[key]
    start = row[0][-1]
    for i, seq in enumerate(row):
        if seq[0] == start + 1:
            start = seq[-1]
        else:
            if not outside:
                for x in range(start + 1, seq[0]):
                    row_oks.append((key, x))
            start = seq[-1]

        if len(seq) == 1:
            outside = not outside

print(row_oks)

# COLUMN-WISE
columns = {}
for x, (i, j) in enumerate(path):
    if j not in columns:
        columns[j] = []
    if x > 0 and path[x - 1][1] == j:
        # same column
        columns[j][-1].append(i)
    else:
        # diff column
        columns[j].append([i])

if path[0][1] == path[-1][1]:
    columns[path[0][1]][0] += columns[path[0][1]][-1]
    columns[path[0][1]][0].sort()
    columns[path[0][1]] = columns[path[0][1]][:-1]

for key in columns:
    for i in columns[key]:
        i.sort()
    columns[key].sort()
print(columns)

column_oks = []
for key in columns:
    outside = True
    columns[key].sort()
    column = columns[key]
    start = column[0][-1]
    for i, seq in enumerate(column):
        if seq[0] == start + 1:
            start = seq[-1]
        else:
            if not outside:
                for x in range(start + 1, seq[0]):
                    column_oks.append((x, key))
            start = seq[-1]

        if len(seq) == 1:
            outside = not outside

print(column_oks)

oks = set(column_oks) & set(row_oks)
print(len(oks))
# print(area)