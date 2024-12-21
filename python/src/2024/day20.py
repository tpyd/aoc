import copy
import heapq

data = """###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"""

with open("input/2024/day20.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split("\n")]

y_max = len(data)
x_max = len(data[0])

start = None
end = None
walls = []
dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]

for y in range(len(data)):
    for x in range(len(data[0])):
        if data[y][x] == "#":
            walls.append((x, y))
        elif data[y][x] == "S":
            start = (x, y)
        elif data[y][x] == "E":
            end = (x, y)
        else:
            pass

def print_board(seen, walls):
    for y in range(len(data)):
        line = ""
        for x in range(len(data[0])):
            if (x, y) in walls:
                line += "#"
            elif (x, y) == start:
                line += "S"
            elif (x, y) == end:
                line += "E"
            elif (x, y) in seen:
                line += "o"
            else:
                line += "."

        print(line)
    

def bfs(start, end, seen):
    heap = []
    heapq.heappush(heap, [0, start, seen])
    
    x, y = start

    while heap:
        length, pos, seen = heapq.heappop(heap)
        x, y = pos
        ex, ey = end
        
        if pos == end:
            return seen

        if pos in seen or pos in walls:
            continue

        if x < 0 or x >= x_max or y < 0 or y >= y_max:
            continue

        for dx, dy in dirs:
            nx, ny = x + dx, y + dy
        
            heapq.heappush(heap, [length + 1, (nx, ny), seen + [(x, y)]])

    return -1
        

current = start
picoseconds = 0
seen = []

path = bfs(start, end, seen) + [end]
#print(len(path))

overview = {}

while current != end:
    #print(f"We are at number {picoseconds+1} / {len(path)}")
    seen.append(current)
    x, y = current

    for cheat_direction in dirs:
        cx, cy = cheat_direction
        nx, ny = x + 2 * cx, y + 2 * cy

        if (nx, ny) not in path:
            continue

        index = path.index((nx, ny))
        remainder = len(path[index:])

        cheat_length = picoseconds + 2 + remainder
        saved = len(path) - 1 - cheat_length

        if saved <= 0:
            continue
        
        if saved not in overview:
            overview[saved] = 1
        else:
            overview[saved] += 1

    # Move forward
    for dx, dy in dirs:
        nx, ny = x + dx, y + dy

        if (nx, ny) in walls or (nx, ny) in seen:
            continue
        
        current = (nx, ny)
        break

    picoseconds += 1


overview = [(k, v) for k, v in overview.items()]
overview = sorted(overview, key=lambda x: x[0])
most_saved = 0

for saved, count in overview:
    #print(f"There are {count} cheats that saves {saved + 1} picoseconds")
    if saved+1 >= 100:
        most_saved += count

print(most_saved)
         
#print("part 2")
def create_cheat_directions(pos):
    max_distance = 20
    x, y = pos

    positions = []

    for nx in range(x - max_distance, x + max_distance + 1):
        for ny in range(y - max_distance, y + max_distance + 1):
            if abs(x - nx) + abs(y - ny) <= max_distance and (nx, ny) != pos:
                positions.append((nx, ny))

    return positions


current = start
picoseconds = 0
seen = []
overview = {}

while current != end:
    #print(f"We are at number {picoseconds+1} / {len(path)}")
    seen.append(current)
    x, y = current

    for cheat_direction in create_cheat_directions(current):
        nx, ny = cheat_direction

        if (nx, ny) not in path:
            continue

        index = path.index((nx, ny))
        remainder = len(path[index:])

        cheat_length = abs(x - nx) + abs(y - ny)

        cheat_length = picoseconds + cheat_length + remainder
        saved = len(path) - 1 - cheat_length

        if saved <= 0:
            continue
        
        if saved not in overview:
            overview[saved] = 1
        else:
            overview[saved] += 1

    # Move forward
    for dx, dy in dirs:
        nx, ny = x + dx, y + dy

        if (nx, ny) in walls or (nx, ny) in seen:
            continue
        
        current = (nx, ny)
        break

    picoseconds += 1


overview = [(k, v) for k, v in overview.items()]
overview = sorted(overview, key=lambda x: x[0])
most_saved = 0

for saved, count in overview:
    #print(f"There are {count} cheats that saves {saved + 1} picoseconds")
    if saved+1 >= 100:
        most_saved += count

print(most_saved)
