data = """########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"""

data = """##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"""

"""#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"""

with open("input/2024/day15.txt") as f:
    data = f.read()

grid, moves = data.strip().split("\n\n")

grid = [list(d) for d in grid.split("\n")]
moves = [m for m in list(moves) if m != "\n"]

y_max = len(grid)
x_max = len(grid[0])

robot = None
boxes = []
walls = []

for y in range(y_max):
    for x in range(x_max):
        if grid[y][x] == '@':
            robot = (x, y)

        if grid[y][x] == 'O':
            boxes.append((x, y))

        if grid[y][x] == '#':
            walls.append((x, y))

dirs = {
    ">": (1, 0),
    "<": (-1, 0),
    "^": (0, -1),
    "v": (0, 1)
}

def print_grid():
    for y in range(y_max):
        line = ""
        for x in range(x_max):
            pos = (x, y)
            if pos == robot:
                line += "@"
            elif pos in boxes:
                line += "O"
            elif pos in walls:
                line += "#"
            else:
                line += "."

        print(line)


for move in moves:
    #print_grid()
    dx, dy = dirs[move] 

    x, y = robot
    skip = False

    while True:
        nx, ny = (x + dx, y + dy)
        x, y = (nx, ny)
        #print(f"Checking {(nx, ny)}")

        if grid[ny][nx] == "#":
            #print(f"Found a wall at {(nx, ny)}, cant move this time")
            skip = True
            break
        
        if (nx, ny) not in boxes and (nx, ny) not in walls:
            #print(f"Found empty spot at {(nx, ny)}, starting to move things")
            break


    if skip:
        continue

    #print(f"We are at {(x, y)}")
    
    while True:
        #print(f"Checking {(x, y)}")
        if (x, y) in boxes:
            #print(f"There is a box here, we have to move it")
            boxes.remove((x, y))
            boxes.append((x + dx, y + dy))

        if (x, y) == robot:
            #print(f"The robot is here, moving it as well")
            robot = (x + dx, y + dy)
            break

        x, y = x - dx, y - dy

    #print("-"*40)
        
#print_grid()

total = 0

for box in boxes:
    x, y = box
    total += x + y * 100

print(total)

# Part 2
robot = None
boxes = []
walls = []

for y in range(y_max):
    for x in range(x_max):
        if grid[y][x] == '@':
            robot = (x*2, y)

        if grid[y][x] == 'O':
            boxes.append((x*2, y, "["))
            boxes.append((x*2+1, y, "]"))

        if grid[y][x] == '#':
            walls.append((x*2, y))
            walls.append((x*2+1, y))

def print_grid():
    for y in range(y_max):
        line = ""
        for x in range(x_max*2):
            pos = (x, y)
            if pos == robot:
                line += "@"
            elif pos in walls:
                line += "#"
            else:
                done = False
                for box in boxes:
                    bx, by, bt = box
                    if (bx, by) == pos:
                        line += bt
                        done = True
                        break
                if done:
                    continue
                line += "."

        print(line)

for i, move in enumerate(moves):
    #print_grid()
    
    dx, dy = dirs[move] 
    #print(f"Moving '{move}'")

    x, y = robot
    skip = False

    # Left or right
    if move in ["<", ">"]:
        while True:
            nx, ny = (x + dx, y + dy)
            x, y = (nx, ny)
            #print(f"Checking {(nx, ny)}")

            # Check for wall, no moving if a wall is found
            if (nx, ny) in walls:
                #print(f"Found a wall at {(nx, ny)}, cant move this time")
                skip = True
                break
            
            # Search for an empty spot
            done = True
            for box in boxes:
                bx, by, _ = box
                if (nx, ny) == (bx, by):
                    #print(f"Found box at {(nx, ny)}")
                    done = False
                    break

            if not done:
                continue
            
            #print(f"Found empty spot at {(nx, ny)}, starting to move things")
            break
        if skip:
            continue

        #print(f"We are at {(x, y)}. MOVING BOXES")
        
        while True:
            x, y = x - dx, y - dy
            #print(f"Have to move {(x-dx, y)} to {(x, y)}")

            for box in boxes:
                bx, by, bt = box
                if (x, y) == (bx, by):
                    #print(f"There is a box here, we have to move it")
                    boxes.remove((x, y, bt))
                    boxes.append((x + dx, y, bt))
                    break

            if (x, y) == robot:
                #print(f"The robot is here, moving it as well")
                robot = (x + dx, y)
                break
    
    if move in ["^", "v"]:
        # Check block above/below robot recursively.
        # If its a [ check to the right also.
        # If its a ] check to the left.
        # If there is one wall, we cant move.
        def can_move(dy, x, y):
            nx, ny = x, y + dy 
            if (nx, ny) in walls:
                return False

            has_box = False
            the_box = None
            for box in boxes:
                bx, by, bt = box
                if (nx, ny) == (bx, by):
                    has_box = True
                    the_box = box

            if has_box:
                # Check more
                bx, by, bt = the_box
                first = can_move(dy, bx, by)
                if bt == "[":
                    second = can_move(dy, bx + 1, by)
                else:
                    second = can_move(dy, bx - 1, by)

                return first and second
            
            return True

        x, y = robot
        moveable = can_move(dy, x, y)

        if moveable:
            def move_pieces(dy, x, y):
                #print(dy, x, y)
                nx, ny = x, y + dy 
                if (nx, ny) in walls:
                    #print("This shouldnt happend")
                    return 

                has_box = False
                the_box = None
                for box in boxes:
                    bx, by, bt = box
                    if (nx, ny) == (bx, by):
                        has_box = True
                        the_box = box

                # Move the box first
                if has_box:
                    # Check more
                    #print(f"Found a box at {(bx, by)} we need to move")
                    bx, by, bt = the_box
                    # Move first piece
                    move_pieces(dy, bx, by)

                    # Move second piece
                    if bt == "[":
                        #print(f"Found left piece")
                        move_pieces(dy, bx + 1, by)
                    else:
                        #print(f"Found right piece")
                        move_pieces(dy, bx - 1, by)

                # Move the original item
                #if (x, y) == robot:
                    #robot = (nx, ny)

                for box in boxes:
                    bx, by, bt = box
                    if (bx, by) == (x, y):
                        boxes.remove((bx, by, bt))
                        boxes.append((nx, ny, bt))

            move_pieces(dy, x, y)
            robot = (x, y + dy)
        
        #print(f"Moveable: {moveable}")

    #print("-"*40)

#print_grid()

total = 0

for box in boxes:
    x, y, t = box
    if t == "[":
        total += x + y * 100

print(total)
