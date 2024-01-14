with open("testcase.txt", "r") as f:
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
    ".": None,
    "S": ("U", "R"), # TODO: Need to change this for different maps so that we can determine the inflection of this point
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
    rows[path[0][0]][0] += rows[path[0][0]][-1]
    rows[path[0][0]][0].sort()
    rows[path[0][0]] = rows[path[0][0]][:-1]

for key in rows:
    t = []
    row = rows[key]
    for x, seq in enumerate(row):
        seq.sort()
        t.append(seq[0])
        
        if len(seq) > 1:
            a = char_to_directions[grid[key][seq[0]]]
            b = char_to_directions[grid[key][seq[-1]]]
            # This checks if both ends are going the same way or not. If they are going different ways, inside = !inside
            # Think: Inflection
            if a[0] == b[0] or a[0] == b[1] or a[1] == b[0] or a[1] == b[1]:
                t.append(seq[-1])

    t.sort()
    rows[key] = t

print(rows)

xs = [p[0] for p in path]
xs.sort()
min_x, max_x = xs[0], xs[-1]


ys = [p[1] for p in path]
ys.sort()
min_y, max_y = ys[0], ys[-1]

area = 0
import bisect
for i in range(min_x, max_x):
    for j in range(min_y, max_y):
        # count number of x intersections
        if (i, j) not in path:
            insertion_point = bisect.bisect_left(rows[i], j)
            intersections = len(rows[i]) - insertion_point
            if (intersections % 2) == 1:
                area += 1

print(area)