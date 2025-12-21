import re

data = """p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"""

with open("input/2024/day14.txt") as f:
    data = f.read()

width = 101
height = 103
seconds = 100

quadrants = [0, 0, 0, 0]

for line in data.strip().split('\n'):
    x, y, dx, dy = map(int, re.findall(r"(\d+),(\d+) v=(-?\d+),(-?\d+)", line)[0])   
    
    x = (x + dx * seconds) % width
    y = (y + dy * seconds) % height

    if x == width // 2 or y == height // 2:
        continue
    
    q = 0
    if x > width / 2:
        q += 1

    if y > height / 2:
        q += 2

    quadrants[q] += 1

prod = 1
for q in quadrants:
    prod *= q

print(prod)

def find_line(positions):
    line_size = 20
    for column_index in range(width):
        column = sorted([p for p in positions if p[0] == column_index], key=lambda x: x[1])
        consecutive = 0

        for x, y in column:
            if (x, y-1) not in column:
                consecutive = 0

            consecutive += 1

            #if consecutive > 6:
                #print(column)

            if consecutive == line_size:
                return True
            
    return False

for i in range(1, seconds + 1 +999999999):
    tree = False
    positions = []

    for line in data.strip().split('\n'):
        x, y, dx, dy = map(int, re.findall(r"(\d+),(\d+) v=(-?\d+),(-?\d+)", line)[0])   
         
        x = (x + dx * i) % width
        y = (y + dy * i) % height

        positions.append((x, y))

    # Check for a long vertical line of bots
    """
    board = ['.'*width for _ in range(height)]
    for x, y in positions:
        line = board[y]
        new_line = line[:x] + 'X' + line[x+1:]
        board[y] = new_line

    print(i)
    for line in board:
        print(line)
    input()
    """
    found = find_line(positions)
    

    if found == True:
        print(i)
        break


