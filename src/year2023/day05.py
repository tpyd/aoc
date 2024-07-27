import re

with open("../../input/2023/real/day05.txt") as f:
    data = f.read()

segments = data.split('\n\n')

# Part 1
seeds = map(int, re.findall(r'\d+', segments[0]))
min_location = float('inf')
    
for seed in seeds:
    for group in segments[1:]:
        for smap in re.findall(r'(\d+) (\d+) (\d+)', group):
            dest, src, rng = map(int, smap)
            if seed >= src and seed < src + rng:
                seed = seed - src + dest
                break

    min_location = min(min_location, seed)

print(min_location)
assert(min_location == 324724204)

# Part 2 
intervals = []

for seed in re.findall(r'(\d+) (\d+)', segments[0]):
    x1, dx = map(int, seed)
    x2 = x1 + dx
    intervals.append((x1, x2, 1))

min_location = float('inf')
while intervals:
    x1, x2, level = intervals.pop()

    if level == 8:
        min_location = min(x1, min_location)
        continue

    for conversion in re.findall(r'(\d+) (\d+) (\d+)', segments[level]):
        z, y1, dy = map(int, conversion)
        y2 = y1 + dy
        diff = z - y1

        # No overlap
        if x2 <= y1 or x1 >= y2:
            continue

        # Overlap on the left side
        if x1 < y1:
            intervals.append((x1, y1, level))
            x1 = y1

        # Overlap on the right side
        if x2 > y2:
            intervals.append((y2, x2, level))
            x2 = y2

        # Pass to next level
        intervals.append((x1 + diff, x2 + diff, level + 1))
        break

    else:
        intervals.append((x1, x2, level + 1))

print(min_location)
assert(min_location == 104070862)

