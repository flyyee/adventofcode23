with open("testcase.txt", "r") as f:
    input = f.read(-1)

def hash(s: str):
    res = 0
    for char in s:
        res += ord(char)
        res *= 17
        res %= 256
    
    return res

ans = 0
for step in input.strip().split(","):
    ans += hash(step)

print(ans)