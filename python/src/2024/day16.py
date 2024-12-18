import copy
import math
import heapq

data = """###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"""

data = """#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"""

with open("input/2024/day16.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split("\n")]

current = None
start = None
end = None

for y in range(len(data)):
    for x in range(len(data[0])):
        if data[y][x] == "S":
            current = (x, y)
            start = (x, y)
        if data[y][x] == "E":
            end = (x, y)

facing = (1, 0)
stack = [(current, 0, facing, [current], [])]

directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

best_paths = []
best_score = 99999999999
counter = 0


def print_path(v, path):
    print("-"*50)
    print(f"Value {v}, length {len(path)}")

    to_print = copy.deepcopy(data)
    for p in path:
        px, py = p
        to_print[py][px] = "O"

    for line in to_print:
        asd = ""
        for point in line:
            asd += point
        print(asd)


"""
while stack:
    if len(stack) == 0:
        print(f"Stack is empty, RIP")
        break

    stack = sorted(stack, key=lambda x: x[1], reverse=True)
    current = stack.pop()
    pos, v, facing, path, seen = current
    x, y = pos
    fx, fy = facing
    new_seen = seen + [(x, y)]

    if v == best_score and pos == end:
        print(f"Found another path with the same score")
        best_paths.append(path)
        continue

    if v > best_score:
        print(f"Stopping, got a path with {v} score")
        break

    if pos == end:
        #print(f"Got to the end at {pos} with {v} points")
        best_score = v
        best_paths.append(path)
        #print_path(v, path)
        continue

    for direction in directions:
        dx, dy = direction
        nx, ny = x + dx, y + dy

        if (nx, ny) in seen:
            continue

        if data[ny][nx] == "#":
            continue

        # Prevent 180
        if (dx, dy) == (fx * -1, fy * -1):
            #print(f"Cant 180, {(dx, dy)} when facing {fx, fy}")
            continue
        
        nv = v + 1
        if direction != facing:
            nv += 1000

        new_path = path + [(nx, ny)]
        
        ex, ey = end
        end_dx, end_dy = (abs(ex - nx), abs(ny - ey))

        stack.append(((nx, ny), nv, direction, new_path, new_seen))


    #print(f"stack has {len(stack)} items")
    counter += 1
    #if counter == 8:
        #break

print(best_score)
#print(len(best_paths))

unique_paths = []
for best in best_paths:
    print_path(best_score, best)
    for p in best:
        if p not in unique_paths:
            unique_paths.append(p)

print(f"Found {len(best_paths)} paths to the exit")
print(f"Total spots: {len(unique_paths)}")
"""

print("- DIJKSTRAS -")

facing = (1, 0)

heap = []
visited = {}
best_score = math.inf
heapq.heappush(heap, (0, start, facing, [start]))
best_paths = []

counter = 0

while heap:
    score, position, direction, path = heapq.heappop(heap)

    #print(f"------- ITERATION {counter} ------")
    #print(f"We are at {position} facing {direction}")

    if score > best_score:
        break

    x, y = position
    dx, dy = direction

    if (position, direction) in visited and visited[(position, direction)] < score:
        #print("Already been here")
        continue

    visited[(position, direction)] = score

    if position == end:
        best_score = score
        best_paths.append(path)

    # Check forward step 
    nx, ny = x + dx, y + dy
    if data[ny][nx] != "#":
        heapq.heappush(heap, (score + 1, (nx, ny), direction, path + [(nx, ny)]))
        #print(f"Added a forward step to {(nx, ny)}")

    # Check sides
    for new_direction in directions:
        # Skip forward
        if new_direction == direction:
            continue

        # Skip 180 degree turns
        if new_direction == (dx * -1, dy * -1):
            continue

        # Look ahead for walls
        adx, ady = new_direction
        ax, ay = x + adx, y + ady
        if data[ay][ax] == "#":
            #print(f"Direction {new_direction} to position {(ax, ay)} is wall")
            continue

        if (x, y, new_direction) in visited:
            #print(f"Direction {new_direction} already visited")
            continue

        ns = score + 1000
        heapq.heappush(heap, (ns, position, new_direction, path + [position]))
        #print(f"Added direction {new_direction}")

    counter += 1
    """
    if counter == 200:
        break
    """

print(best_score)

unique_points = []
for path in best_paths:
    for point in path:
        if point not in unique_points:
            unique_points.append(point)

print(len(unique_points))

# Backtrack from end to start
"""
current_pos = end
path = []

while current_pos != start:
    parents, node_score = nodes[current_pos]
    p = parents[0]
    path.append(p)

    print_path(0, path)

    current_pos = p

print(path)
    
"""
