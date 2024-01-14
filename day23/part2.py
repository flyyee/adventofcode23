from enum import Enum
import sys
sys.setrecursionlimit(20000)  # lol

with open("testcase.txt", "r") as f:
    input = f.read(-1)

Tile = Enum('Tile', ["FOREST", "PATH"])

char_to_tile = {'#': Tile.FOREST, '.': Tile.PATH, '^': Tile.PATH, '>': Tile.PATH, 'v': Tile.PATH, '<': Tile.PATH}

rows = input.split("\n")
grid = [[char_to_tile[tile] for tile in row] for row in rows]
for j, tile in enumerate(grid[0]):
    if tile is Tile.PATH:
        start_pos = (0, j)
        break
else:
    print("Could not find start pos")
    exit()

for j, tile in enumerate(grid[-1]):
    if tile is Tile.PATH:
        end_pos = (len(grid) - 1, j)
        break
else:
    print("Could not find end pos")
    exit()

# print(start_pos, end_pos)

# Based on the heuristic that most tiles only have one possible way in/out
# We convert the provided grid into a graph of junctions as nodes, and edges containing sequences of such nodes
# Then, we can just run DFS

class Node:
    def __init__(self, name = None):
        self.connections = {}
        self.name = name

    def add_connection_to(self, node, weight):
        if node in self.connections:
            self.connections[node] = max(weight, self.connections[node])
        else:
            self.connections[node] = weight

    def add_connection_bidirectional(self, node, weight_to, weight_from = None):
        if weight_from is None:
            weight_from = weight_to
        self.add_connection_to(node, weight_to)
        node.add_connection_to(self, weight_from)

start_node = Node("start")
end_node = Node("end")
junction_positions = []
junction_nodes = []

for i, row in enumerate(grid):
    for j, tile in enumerate(row):
        if tile is not Tile.PATH:
            continue
        
        offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        valid_offsets = []
        for offset in offsets:
            new_pos = (i + offset[0], j + offset[1])
            # Check in bounds
            if new_pos[0] < 0 or new_pos[0] >= len(grid) or new_pos[1] < 0 or new_pos[1] >= len(grid[0]):
                continue
            # Check is path / valid slope
            if grid[new_pos[0]][new_pos[1]] is Tile.FOREST:
                continue
            
            valid_offsets.append(offset)

        if len(valid_offsets) <= 2:
            continue
    
        # JUNCTION
        junction_positions.append((i, j))
        junction_nodes.append(Node(len(junction_positions) - 1))

for pos, node in zip(junction_positions, junction_nodes):
    # traverse from each pos till we reach another junction, then add the connection
    offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    valid_offsets = []
    for offset in offsets:
        new_pos = (pos[0] + offset[0], pos[1] + offset[1])
        # Check in bounds
        if new_pos[0] < 0 or new_pos[0] >= len(grid) or new_pos[1] < 0 or new_pos[1] >= len(grid[0]):
            continue
        # Check is path / valid slope
        if grid[new_pos[0]][new_pos[1]] is Tile.FOREST:
            continue
        
        current_path_length = 1

        saved_pos = pos

        prev_pos = pos
        pos = new_pos
        # Valid new_pos
        while pos not in junction_positions and pos != start_pos and pos != end_pos:
            for offset in offsets:
                new_pos = (pos[0] + offset[0], pos[1] + offset[1])
                # Check in bounds
                if new_pos[0] < 0 or new_pos[0] >= len(grid) or new_pos[1] < 0 or new_pos[1] >= len(grid[0]):
                    continue
                # Check is path / valid slope
                if grid[new_pos[0]][new_pos[1]] is Tile.FOREST:
                    continue
                if new_pos == prev_pos:
                    continue
                break
            
            prev_pos = pos
            pos = new_pos
            current_path_length += 1
            
        # new_pos is a junction_position
        if pos == start_pos:
            node.add_connection_bidirectional(start_node, current_path_length)
        elif pos == end_pos:
            node.add_connection_bidirectional(end_node, current_path_length)
        else:
            junction_node = junction_nodes[junction_positions.index(pos)]
            node.add_connection_bidirectional(junction_node, current_path_length)

        pos = saved_pos

# print("debug")
# print("start: ", end="")
# for connection in start_node.connections.keys():
#     print(str(connection.name) + ', ', end = "")
# print("")

# for pos, node in zip(junction_positions, junction_nodes):
#     print(str(node.name) + f" {pos}: ", end = "")
#     for connection in node.connections.keys():
#         print(str(connection.name) + ', ', end = "")
#     print("")

# print("end: ", end="")
# for connection in end_node.connections.keys():
#     print(str(connection.name) + ', ', end = "")
# print("")

# DFS time
longest_path = 0
path = set()
node = start_node
path_length = 0

print("beginning DFS")
# run DFS from start_pos => end_pos
def dfs():
    global longest_path, path, node, path_length
    if node == end_node:
        # Win condition
        longest_path = max(longest_path, path_length)
        return

    for adj_node, weight in node.connections.items():
        if adj_node in path:
            continue
        path.add(adj_node)
        path_length += weight
        old_node = node
        node = adj_node
        dfs()
        node = old_node
        path_length -= weight
        path.remove(adj_node)

dfs()
print(f"{longest_path=}")