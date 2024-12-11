data = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"""

with open("input/2024/day10.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split("\n")]
y_max = len(data)
x_max = len(data[0])

trailheads = []

for y in range(y_max):
    for x in range(x_max):
        if data[y][x] == '0':
            trailheads.append((x, y))

def look_around(point, elevation):
    trail_paths = []

    directions = [(0, 1), (0, -1), (-1, 0), (1, 0)]
    for d in directions:
        new_loc = (point[0] + d[0], point[1] + d[1])
        if 0 <= new_loc[0] < x_max and 0 <= new_loc[1] < y_max:
            new_elevation = int(data[new_loc[1]][new_loc[0]])
            if new_elevation == elevation + 1:
                trail_paths.append((new_loc, new_elevation))

    #new_locs = [(point[0] + d[0], point[1] + d[1]) for d in directions]
    #new_locs = [p for p in new_locs if int(data[p[1]][p[0]]) == elevation + 1]
    #new_locs = [(p, elevation + 1) for p in new_locs if 0 <= p[0] <= x_max and 0 <= p[1] <= y_max]

    return trail_paths

total = 0

for trailhead in trailheads:
    score = 0
    seen = []

    next_paths = [(trailhead, 0)]

    while next_paths:
        step, elevation = next_paths.pop()

        if elevation == 9:
            score += 1
            continue

        new_steps = look_around(step, elevation)

        for s, e in new_steps:
            if s in seen:
                continue

            seen.append(s)
            next_paths.append((s, e)) 

    total += score

print(total)

total_rating = 0

for trailhead in trailheads:
    score = 0
    seen = []

    next_paths = [(trailhead, 0)]

    while next_paths:
        step, elevation = next_paths.pop()

        if elevation == 9:
            score += 1
            continue

        new_steps = look_around(step, elevation)

        for s, e in new_steps:
            #if s in seen:
                #continue

            seen.append(s)
            next_paths.append((s, e)) 

    total_rating += score

print(total_rating)

