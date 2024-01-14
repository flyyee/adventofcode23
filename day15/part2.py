with open("testcase.txt", "r") as f:
    input = f.read(-1)

def hash(s: str):
    res = 0
    for char in s:
        res += ord(char)
        res *= 17
        res %= 256
    
    return res

boxes = [[] for _ in range(256)]

for step in input.strip().split(","):
    if step[-1] == "-":
        label = step[:-1]
        boxno = hash(label)
        boxes[boxno] = [q for q in boxes[boxno] if q[0] != label]

    else:
        focal = step[-1]
        label = step[:-2]
        boxno = hash(label)
        for q in boxes[boxno]:
            if q[0] == label:
                q[1] = focal
                break
        else:
            boxes[boxno].append([label, focal])

ans = 0
for i, box in enumerate(boxes):
    for j, (_, focal) in enumerate(box):
        ans += (i + 1) * (j + 1) * int(focal)

print(ans)