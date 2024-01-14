def search(val, l, comp):
    low = 0
    high = len(l) - 1
    while low <= high:
        mid = (low + high) // 2
        print(mid, low, high)
        comp_result = comp(val, l[mid])
        if comp_result > 0:
            if mid == len(l) - 1:
                return len(l)

            if comp(val, l[mid + 1]) < 0:
                return mid + 1

            low = mid + 1
        else:
            if mid == 0:
                return 0
            
            if comp(val, l[mid - 1]) > 0:
                return mid
            
            high = mid - 1
    
    return (low + high) // 2


ans = search(6, [1, 3, 4, 5], lambda x, y: 1 if x > y else 0)
print(f"{ans=}")