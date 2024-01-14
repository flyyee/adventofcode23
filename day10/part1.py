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
        print(path)

        print(len(path) // 2)
        break