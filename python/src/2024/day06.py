import copy

with open("input/2024/day06.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split('\n')]

y_max = len(data) - 1
x_max = len(data[0]) - 1

starting_position = None

for y, _ in enumerate(data):
    for x, d in enumerate(data[y]):
        if d == "^":
            starting_position = (x, y)

guard = starting_position
visited = set()
visited.add(guard)
direction = "up"
next_position = (guard[0], guard[1] - 1)

def next(guard, direction):
    if direction == "left":
        return (guard[0] - 1, guard[1])
    if direction == "right":
        return (guard[0] + 1, guard[1])
    if direction == "down":
        return (guard[0], guard[1] + 1)
    if direction == "up":
        return (guard[0], guard[1] - 1)

while True:
    next_position = next(guard, direction)

    if next_position[0] < 0 or next_position[0] > x_max or next_position[1] < 0 or next_position[1] > y_max:
        break
    
    if data[next_position[1]][next_position[0]] == "#":
        if direction == "up":
            direction = "right"
        elif direction == "right":
            direction = "down"
        elif direction == "down":
            direction = "left"
        elif direction == "left":
            direction = "up"

        next_position = next(guard, direction)

    if next_position[0] < 0 or next_position[0] > x_max or next_position[1] < 0 or next_position[1] > y_max:
        break

    guard = next_position
    visited.add(guard)

print(len(visited))

starting_direction = "up"
num_loops = 0

for pos in visited:
    data_copy = copy.deepcopy(data)
    data_copy[pos[1]][pos[0]] = "#"

    visited = set()
    guard = starting_position
    direction = starting_direction
    visited.add((guard, direction))

    while True:
        next_position = next(guard, direction)

        if next_position[0] < 0 or next_position[0] > x_max or next_position[1] < 0 or next_position[1] > y_max:
            break
        
        while data_copy[next_position[1]][next_position[0]] == "#":
            if direction == "up":
                direction = "right"
            elif direction == "right":
                direction = "down"
            elif direction == "down":
                direction = "left"
            elif direction == "left":
                direction = "up"

            next_position = next(guard, direction)

        if next_position[0] < 0 or next_position[0] > x_max or next_position[1] < 0 or next_position[1] > y_max:
            break

        guard = next_position
        if (guard, direction) in visited:
            num_loops += 1
            break
        visited.add((guard, direction))

print(num_loops)
     

