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
        somemap[loc[1]][loc[0]] = newsteps
        prevloc = loc
        loc = nextloc

    validlocs.extend(tempvalidlocs)

maxsteps = max([max(s) for s in somemap])
print(maxsteps)

