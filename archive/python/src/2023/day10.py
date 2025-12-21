test1 = """
.....
.S-7.
.|.|.
.L-J.
.....
"""

test2 = """
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"""

test3 = """
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"""

test4 = """
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"""

test5 = """
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"""

test6 = """
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"""

test7 = """
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"""

with open("../../input/2023/real/day10.txt", "r") as f:
    real = f.read()

data = real

parsed = []
data = data.strip()
for line in data.split('\n'):
    linedata = []
    strippedline = line.strip()
    for char in strippedline:
        linedata.append(char)
    parsed.append(linedata)

height = len(parsed)
width = len(parsed[0])

# Find the S
def find_s():
    for y in range(height):
        for x in range(width):
            d = parsed[y][x]
            if d == 'S':
                return (x, y)

start = find_s()
somemap = []
for y in range(height):
    somemap.append([])
    for x in range(width):
        somemap[y].append(-1)

# Try all directions from S
# North, west, south, east
dirs = [(start[0], start[1] - 1), (start[0] - 1, start[1]), (start[0], start[1] + 1), (start[0] + 1, start[1])]

validlocs = []
pipelocs = []

for loc in dirs:
    prevloc = start
    steps = 0
    tempvalidlocs = []
    invalid = False

    while True:
        steps += 1
        
        pipe = parsed[loc[1]][loc[0]]

        # go one direction unless we've already been there, then go the other direction
        if pipe == '|':
            if prevloc == (loc[0] - 1, loc[1]) or prevloc == (loc[0] + 1, loc[1]):
                break
            nextloc = (loc[0], loc[1] - 1)
            if nextloc == prevloc:
                nextloc = (loc[0], loc[1] + 1)
        if pipe == '-':
            if prevloc == (loc[0], loc[1] - 1) or prevloc == (loc[0], loc[1] + 1):
                break
            nextloc = (loc[0] + 1, loc[1])
            if nextloc == prevloc:
                nextloc = (loc[0] - 1, loc[1])
        if pipe == 'L':
            if prevloc == (loc[0] - 1, loc[1]) or prevloc == (loc[0], loc[1] + 1):
                break
            nextloc = (loc[0] + 1, loc[1])
            if nextloc == prevloc:
                nextloc = (loc[0], loc[1] - 1)
        if pipe == 'J':
            if prevloc == (loc[0] + 1, loc[1]) or prevloc == (loc[0], loc[1] + 1):
                break
            nextloc = (loc[0] - 1, loc[1])
            if nextloc == prevloc:
                nextloc = (loc[0], loc[1] - 1)
        if pipe == '7':
            if prevloc == (loc[0] + 1, loc[1]) or prevloc == (loc[0], loc[1] - 1):
                break
            nextloc = (loc[0] - 1, loc[1])
            if nextloc == prevloc:
                nextloc = (loc[0], loc[1] + 1)
        if pipe == 'F':
            if prevloc == (loc[0] - 1, loc[1]) or prevloc == (loc[0], loc[1] - 1):
                break
            nextloc = (loc[0] + 1, loc[1])
            if nextloc == prevloc:
                nextloc = (loc[0], loc[1] + 1)
        if pipe == 'S':
            break
        if pipe == '.':
            invalid = True
            break
 
        tempvalidlocs.append((loc[0], loc[1], steps))
        newsteps = steps
        if somemap[loc[1]][loc[0]] != -1:
            newsteps = min(steps, somemap[loc[1]][loc[0]])
            pipelocs.append((loc[0], loc[1]))
        somemap[loc[1]][loc[0]] = newsteps
        prevloc = loc
        loc = nextloc

    validlocs.extend(tempvalidlocs)

maxsteps = max([max(s) for s in somemap])
print(maxsteps)

# Part 2
start_shape = None

sx, sy = start
first = [v for v in validlocs if v[2] == 1]

leftside = set()
rightside = set()

from enum import Enum
class Direction(Enum):
    UP = (0, -1)
    DOWN = (0, 1)
    LEFT = (-1, 0)
    RIGHT = (1, 0)

    @classmethod
    def from_tuple(cls, loc):
        for direction in cls:
            if direction.value == loc:
                return direction
        raise ValueError()

prevloc = start
first = first[0]
loc = (first[0], first[1])
while loc[0] != start[0] or loc[1] != start[1]:
    direction = Direction.from_tuple((loc[0] - prevloc[0], loc[1] - prevloc[1]))
    pipe = parsed[loc[1]][loc[0]]

    if pipe == '|':
        if direction == Direction.UP:
            rightside.add((loc[0] + 1, loc[1])) 
            leftside.add((loc[0] - 1, loc[1])) 
        else:
            rightside.add((loc[0] - 1, loc[1])) 
            leftside.add((loc[0] + 1, loc[1])) 
        nextloc = (loc[0], loc[1] - 1)
        if nextloc == prevloc:
            nextloc = (loc[0], loc[1] + 1)
    elif pipe == '-':
        if direction == Direction.RIGHT:
            rightside.add((loc[0], loc[1] + 1)) 
            leftside.add((loc[0], loc[1] - 1)) 
        else:
            rightside.add((loc[0], loc[1] - 1)) 
            leftside.add((loc[0], loc[1] + 1)) 
        nextloc = (loc[0] + 1, loc[1])
        if nextloc == prevloc:
            nextloc = (loc[0] - 1, loc[1])
    elif pipe == 'L':
        if direction == Direction.LEFT:
            leftside.add((loc[0], loc[1] + 1)) 
            leftside.add((loc[0] + 1, loc[1])) 
        else:
            rightside.add((loc[0] + 1, loc[1])) 
            rightside.add((loc[0], loc[1] + 1)) 
        nextloc = (loc[0] + 1, loc[1])
        if nextloc == prevloc:
            nextloc = (loc[0], loc[1] - 1)
    elif pipe == 'J':
        if direction == Direction.RIGHT:
            rightside.add((loc[0], loc[1] + 1)) 
            rightside.add((loc[0] + 1, loc[1])) 
        else:
            leftside.add((loc[0] + 1, loc[1])) 
            leftside.add((loc[0], loc[1] + 1)) 
        nextloc = (loc[0] - 1, loc[1])
        if nextloc == prevloc:
            nextloc = (loc[0], loc[1] - 1)
    elif pipe == '7':
        if direction == Direction.RIGHT:
            leftside.add((loc[0], loc[1] - 1)) 
            leftside.add((loc[0] + 1, loc[1])) 
        else:
            rightside.add((loc[0] + 1, loc[1])) 
            rightside.add((loc[0], loc[1] - 1)) 
        nextloc = (loc[0] - 1, loc[1])
        if nextloc == prevloc:
            nextloc = (loc[0], loc[1] + 1)
    elif pipe == 'F':
        if direction == Direction.LEFT:
            rightside.add((loc[0], loc[1] - 1)) 
            rightside.add((loc[0] - 1, loc[1])) 
        else:
            leftside.add((loc[0] - 1, loc[1])) 
            leftside.add((loc[0], loc[1] - 1)) 
        nextloc = (loc[0] + 1, loc[1])
        if nextloc == prevloc:
            nextloc = (loc[0], loc[1] + 1)
    elif pipe == 'S':
        break
    elif pipe == '.':
        raise ValueError("Something")

    prevloc = loc
    loc = nextloc

pipelocs = [(v[0], v[1]) for v in validlocs]

def expand(starts):
    expanded = set(starts)
    tocheck = set(starts)

    while len(tocheck) > 0:
        loc = tocheck.pop() 

        around = [
            (loc[0], loc[1] + 1),
            (loc[0], loc[1] - 1),
            (loc[0] - 1, loc[1]),
            (loc[0] + 1, loc[1]),
        ]

        for ar in around:
            if ar in pipelocs:
                continue

            if ar in expanded:
                continue

            # Bounds check
            if ar[0] == 0 or ar[1] == 0 or ar[0] == width - 1 or ar[1] == height - 1:
                return None

            expanded.add(ar)
            tocheck.add(ar)

    return expanded

leftside = [l for l in leftside if l not in pipelocs and l != start]
rightside = [r for r in rightside if r not in pipelocs and r != start]

lefts = expand(leftside)
rights = expand(rightside)

if lefts is not None:
    print(len(lefts))
else:
    print(len(rights))

