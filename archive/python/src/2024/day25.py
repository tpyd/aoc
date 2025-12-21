data = """#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"""

with open("input/2024/day25.txt") as f:
    data = f.read()

data = data.strip().split("\n\n")

locks = []
keys = []

for block in data:
    lines = block.split("\n")
    columns = [0, 0, 0, 0, 0]

    # Lock
    if lines[0][0] == "#":
        for line in lines[1:-1]:
            for i, v in enumerate(line):
                if v == "#":
                    columns[i] += 1

        locks.append(columns)
        

    # Key
    if lines[0][0] == ".":
        for line in lines[1:-1]:
            for i, v in enumerate(line):
                if v == "#":
                    columns[i] += 1

        keys.append(columns)

def fits(lock, key):
    for l, k in zip(lock, key):
        if l + k > 5:
            return False

    return True


num_fits = 0

for lock in locks:
    for key in keys:
        if fits(lock, key):
            num_fits += 1

print(num_fits)
