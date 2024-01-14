from enum import Enum
import sys
sys.setrecursionlimit(20000)  # lol

with open("testcase.txt", "r") as f:
    input = f.read(-1)

Tile = Enum('Tile', ["FOREST", "PATH", "UPSLOPE", "RIGHTSLOPE", "DOWNSLOPE", "LEFTSLOPE"])

char_to_tile = {'#': Tile.FOREST, '.': Tile.PATH, '^': Tile.UPSLOPE, '>': Tile.RIGHTSLOPE, 'v': Tile.DOWNSLOPE, '<': Tile.LEFTSLOPE}

rows = input.split("\n")
grid = [[char_to_tile[tile] for tile in row] for row in rows]
for j, tile in enumerate(grid[0]):
    if tile is Tile.PATH:
        start_pos = [0, j]
        break
else:
    print("Could not find start pos")
    exit()

for j, tile in enumerate(grid[-1]):
    if tile is Tile.PATH:
        end_pos = [len(grid) - 1, j]
        break
else:
    print("Could not find end pos")
    exit()

print(start_pos, end_pos)

longest_path = 0
Position = list[int]
# run DFS from start_pos => end_pos
def dfs(pos: Position, path: list[Position]):
    global longest_path
    # Find possible from 
    if pos == end_pos:
        # Win condition
        longest_path = max(longest_path, len(path))
        return
    
    # Check if pos is slope
    slopes = [Tile.UPSLOPE, Tile.RIGHTSLOPE, Tile.DOWNSLOPE, Tile.LEFTSLOPE]
    if grid[pos[0]][pos[1]] in slopes:
        # only one way we can go
        offsets = {
            Tile.UPSLOPE: [-1, 0],
            Tile.DOWNSLOPE: [1, 0],
            Tile.LEFTSLOPE: [0, -1],
            Tile.RIGHTSLOPE: [0, 1]
        }
        offsets = [offsets[grid[pos[0]][pos[1]]]]
    else:
        offsets = [[1, 0], [-1, 0], [0, 1], [0, -1]]

    for offset in offsets:
        new_pos = [pos[0] + offset[0], pos[1] + offset[1]]
        # Check in bounds
        if new_pos[0] < 0 or new_pos[0] >= len(grid) or new_pos[1] < 0 or new_pos[1] >= len(grid[0]):
            continue
        # Check is path / valid slope
        if grid[new_pos[0]][new_pos[1]] is Tile.FOREST:
            continue
        # Check is not travelled before
        if new_pos in path:
            continue
        
        # Continue using it with DFS
        path.append(new_pos)
        dfs(new_pos, path)
        path.pop()

dfs(start_pos, [])
print(f"{longest_path=}")