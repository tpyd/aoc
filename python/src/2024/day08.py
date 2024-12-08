with open("input/2024/day08.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split("\n")]
y_max = len(data)
x_max = len(data[0])

antennas = {}

for y in range(y_max):
    for x in range(x_max):
        symbol = data[y][x]

        if symbol == ".":
            continue

        if symbol not in antennas:
            antennas[symbol] = []

        antennas[symbol].append((x, y))

antinodes = set()

for positions in antennas.values():
    for i, first in enumerate(positions[:-1]):
        for second in positions[i+1:]:
            diff = (second[0] - first[0], second[1] - first[1])
            antinode_1 = (first[0] - diff[0], first[1] - diff[1])
            antinode_2 = (second[0] + diff[0], second[1] + diff[1])

            if 0 <= antinode_1[0] < x_max and 0 <= antinode_1[1] < y_max:
                antinodes.add(antinode_1) 

            if 0 <= antinode_2[0] < x_max and 0 <= antinode_2[1] < y_max:
                antinodes.add(antinode_2) 

print(len(antinodes))

antinodes = set()

for positions in antennas.values():
    for i, first in enumerate(positions[:-1]):
        for second in positions[i+1:]:
            diff = (second[0] - first[0], second[1] - first[1])

            x, y = diff
            while 0 <= second[0] - x < x_max and 0 <= second[1] - y < y_max:
                antinode = (second[0] - x, second[1] - y)
                antinodes.add(antinode)
                x += diff[0]
                y += diff[1]

            x, y = diff
            while 0 <= first[0] + x < x_max and 0 <= first[1] + y < y_max:
                antinode = (first[0] + x, first[1] + y)
                antinodes.add(antinode)
                x += diff[0]
                y += diff[1]

print(len(antinodes))
