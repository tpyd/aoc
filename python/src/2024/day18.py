import heapq
import math

data = """5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"""

with open("input/2024/day18.txt") as f:
    data = f.read()

data = [tuple(map(int, d.split(","))) for d in data.strip().split("\n")]
byte_list = data[:1024]

x_max = 71
y_max = 71

start = (0, 0)
end = (x_max - 1, y_max - 1)

heap = []
seen = {}
best_score = math.inf
best_path = None

heapq.heappush(heap, [0, start, []])

dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]

def print_grid(pos, path):
    print("-"*20)
    for y in range(y_max):
        line = ""
        for x in range(x_max):
            p = (x, y)

            if p == pos:
                line += "X"
            elif p in path:
                line += "o"
            elif p in byte_list:
                line += "#"
            else:
                line += "."

        print(line)

while heap:
    v, pos, path = heapq.heappop(heap)
    path = path + [pos]

    if pos in seen:
        continue

    if pos == end:
        best_score = v
        best_path = path
        break

    seen[pos] = v

    x, y = pos
    
    for dx, dy in dirs:
        nx, ny = x + dx, y + dy 
        #print(f"Testing from {(x, y)} to {(nx, ny)}")

        if nx < 0 or nx >= x_max or ny < 0 or ny >= y_max:
            #print("Outside")
            continue

        if (nx, ny) in seen:
            #print("Been there")
            continue

        if (nx, ny) in byte_list:
            #print("Wall")
            continue

        ex, ey = end[0] - nx, end[1] - ny
        heapq.heappush(heap, [v + 1, (nx, ny), path])

print_grid(pos, path)
print(best_score)

# Part 2
for i in range(3030, len(data)):
    print(f"Checking {i}")
    byte_list = data[:i]
    start = (0, 0)
    end = (x_max - 1, y_max - 1)

    heap = []
    seen = {}
    best_score = math.inf
    best_path = None

    heapq.heappush(heap, [0, start, []])

    dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]

    while heap:
        v, pos, path = heapq.heappop(heap)
        path = path + [pos]

        if pos in seen:
            continue

        if pos == end:
            best_score = v
            best_path = path
            break

        seen[pos] = v

        x, y = pos
        
        for dx, dy in dirs:
            nx, ny = x + dx, y + dy 
            #print(f"Testing from {(x, y)} to {(nx, ny)}")

            if nx < 0 or nx >= x_max or ny < 0 or ny >= y_max:
                #print("Outside")
                continue

            if (nx, ny) in seen:
                #print("Been there")
                continue

            if (nx, ny) in byte_list:
                #print("Wall")
                continue

            ex, ey = end[0] - nx, end[1] - ny
            heapq.heappush(heap, [v + 1, (nx, ny), path])

    if best_path is None:
        break

    #print_grid(pos, path)
    #print(best_score)

print(i-1)
print(data[i-1])
