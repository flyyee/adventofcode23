def shoelace(vertices):
    area = 0
    n = len(vertices)
    for i in range(1, n + 1):
        area += vertices[i % n][1] * (vertices[(i - 1) % n][0] - vertices[(i + 1) % n][0])
    return area / 2

ans = shoelace([(1,6), (3,1), (7,2), (4,4), (8,5)])
print(ans)