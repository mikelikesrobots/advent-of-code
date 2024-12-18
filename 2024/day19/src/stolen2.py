# Source: https://www.youtube.com/watch?v=enX0gRgfuoo

towels = set()

cache = {}
def solve(s):
    if s not in cache:
        if len(s) == 0:
            return 1
        else:
            result = 0
            for poss in towels:
                result += solve(s[len(poss):])
            cache[s] = result
    return cache[s]
