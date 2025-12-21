data = """AAAA
BBCD
BBCC
EEEC
"""

data = """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"""

with open("input/2024/day12.txt") as f:
    data = f.read()

data = [list(d) for d in data.strip().split("\n")]
y_max = len(data)
x_max = len(data[0])

regions = []
dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]

def is_neighbour(point, to_check):
    x, y = point

    for n in to_check:
        nx, ny = n

        for d in dirs:
            dx, dy = d
            
            if x + dx == nx and y + dy == ny:
                return True 

def get_neighbours(point, plant_type):
    x, y = point
    neighbours = []

    for d in dirs:
        dx, dy = d
        nx = x + dx
        ny = y + dy

        if 0 <= nx < x_max and 0 <= ny < y_max and plant_type == data[ny][nx]:
            neighbours.append((nx, ny))

    return neighbours

def expand(point, plant_type):
    region_points = []
    to_check = [point]
    
    while True:
        #print(region_points)
        if len(to_check) == 0:
            break

        p = to_check.pop()
        region_points.append(p)
        neighbours = get_neighbours(p, plant_type) 
        #print(f"checking neighbours {neighbours}")

        for n in neighbours:
            if n not in region_points and n not in to_check:
                #print(f"Pint {n} made it in")
                to_check.append(n)

    return region_points
    

for y in range(len(data)):
    for x in range(len(data[0])):
        plant_type = data[y][x]
        point = (x, y)

        new_region = True
        #print(f"Gonna check {point}")

        for region in regions:
            region_plant_type, region_points = region

            if plant_type != region_plant_type:
                #print(f"It point {point} WRONG in region {region}")
                continue

            if point in region_points:
                #print(f"It point {point} in region {region}")
                new_region = False
            
        if new_region:
            new_region_points = expand(point, plant_type)
            regions.append((plant_type, new_region_points))
            #print(f"Created region {plant_type} with points {new_region_points}")
        
price = 0

for region in regions:
    region_plant_type, region_points = region
    area = len(region_points)
    perimiters = 0

    for point in region_points:
        nb = get_neighbours(point, region_plant_type)
        perimiters += 4 - len(nb)

    price += area * perimiters

print(price)

price = 0

for region in regions:
    region_plant_type, region_points = region
    area = len(region_points)
    
    min_left = min([p[0] for p in region_points])
    max_left = max([p[0] for p in region_points])
    min_top = min([p[1] for p in region_points])
    max_top = max([p[1] for p in region_points])

    total_sides = 0

    # Left to right scan
    left_sides = 0
    for i in range(min_left, max_left + 1):
        sides = 0
        points = sorted([p for p in region_points if p[0] == i], key=lambda x: x[1])

        current_points = [] 
        #print(f"Checking type {region_plant_type} column {i}: points: {points}")
        for p in points:
            #print(f"Checking {p}")
            over_point = (p[0], p[1] - 1)
            left_point = (p[0] - 1, p[1])

            if 0 <= left_point[0] < x_max and data[left_point[1]][left_point[0]] == region_plant_type:
                #print(f"Cant add, {region_plant_type} is on the left side")
                continue

            if over_point not in current_points:
                #print(f"above is not {region_plant_type}, new side discovered.")
                sides += 1
                current_points = [p]
                continue

            #print(f"Nothing new here..")
            current_points.append(p) 
        #print(f"Found {sides} sides in column {i} of type {region_plant_type}")
        left_sides += sides

    #print(f"Found {total_sides} left sides for {region_plant_type}")

    # Right to left scan
    right_sides = 0
    for i in range(max_left, min_left - 1, -1):
        sides = 0
        points = sorted([p for p in region_points if p[0] == i], key=lambda x: x[1])

        current_points = [] 
        #print(f"Checking type {region_plant_type} column {i}: points: {points}")
        for p in points:
            #print(f"Checking {p}")
            over_point = (p[0], p[1] - 1)
            right_point = (p[0] + 1, p[1])

            if 0 <= right_point[0] < x_max and data[right_point[1]][right_point[0]] == region_plant_type:
                #print(f"Cant add, {region_plant_type} is on the right side")
                continue

            if over_point not in current_points:
                #print(f"above is not {region_plant_type}, new side discovered.")
                sides += 1
                current_points = [p]
                continue

            #print(f"Nothing new here..")
            current_points.append(p) 
        #print(f"Found {sides} sides in column {i} of type {region_plant_type}")
        right_sides += sides

    #print(f"Found {total_sides} right sides for {region_plant_type}")

    min_left = min([p[0] for p in region_points])
    max_left = max([p[0] for p in region_points])
    min_top = min([p[1] for p in region_points])
    max_top = max([p[1] for p in region_points])
            
    # Top to down
    top_sides = 0
    for i in range(min_top, max_top + 1):
        sides = 0
        points = sorted([p for p in region_points if p[1] == i], key=lambda x: x[0])

        current_points = [] 
        #print(f"Checking type {region_plant_type} column {i}: points: {points}")
        for p in points:
            #print(f"Checking {p}")
            left_point = (p[0] - 1, p[1])
            over_point = (p[0], p[1] - 1)

            if 0 <= over_point[1] < y_max and data[over_point[1]][over_point[0]] == region_plant_type:
                #print(f"Cant add, {region_plant_type} is above")
                continue

            if left_point not in current_points:
                #print(f"left side is not {region_plant_type}, new side discovered.")
                sides += 1
                current_points = [p]
                continue

            #print(f"Nothing new here..")
            current_points.append(p) 
        #print(f"Found {sides} sides in column {i} of type {region_plant_type}")
        top_sides += sides

    #print(f"Found {total_sides} top sides for {region_plant_type}")

    # Bottom up scan
    bottom_sides = 0
    for i in range(max_top, min_top - 1, -1):
        sides = 0
        points = sorted([p for p in region_points if p[1] == i], key=lambda x: x[0])

        current_points = [] 
        #print(f"Checking type {region_plant_type} column {i}: points: {points}")
        for p in points:
            #print(f"Checking {p}")
            left_point = (p[0] - 1, p[1])
            under_point = (p[0], p[1] + 1)

            if 0 <= under_point[1] < y_max and data[under_point[1]][under_point[0]] == region_plant_type:
                #print(f"Cant add, {region_plant_type} is below")
                continue

            if left_point not in current_points:
                #print(f"left side is not {region_plant_type}, new side discovered.")
                sides += 1
                current_points = [p]
                continue

            #print(f"Nothing new here..")
            current_points.append(p) 
        #print(f"Found {sides} sides in column {i} of type {region_plant_type}")
        bottom_sides += sides

    #print(f"Found {total_sides} bottom sides for {region_plant_type}")

    total_sides = left_sides + right_sides + bottom_sides + top_sides
    price += area * total_sides        

print(price)
    
