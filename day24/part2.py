from z3 import *
with open("testcase.txt", "r") as f:
    input = f.read(-1)

class Hailstone:
    def __init__(self, line: str):
        position, velocity = line.split(" @ ")
        self.position = [int(x) for x in position.split(", ")]
        self.velocity = [int(x) for x in velocity.split(", ")]

hailstones = [Hailstone(line) for line in input.split("\n")]

rock_position = (Real('position_rock_x'), Real('position_rock_y'), Real('position_rock_z'))
rock_velocity = (Real('velocity_rock_x'), Real('velocity_rock_y'), Real('velocity_rock_z'))
s = Solver()
for i, hailstone in enumerate(hailstones):
    if i > 7:
        break
    l = Real(f'lambda_{i}')
    s.add(l >= 0)
    for j in range(3):
        s.add(rock_position[j] + l * rock_velocity[j] == hailstone.position[j] + hailstone.velocity[j] * l)

if s.check() == sat:
    solution = s.model()
    ans = 0.0
    for x in rock_position:
        ans += float(str(solution[x]))
    print(ans)
else:
    print("unsat")