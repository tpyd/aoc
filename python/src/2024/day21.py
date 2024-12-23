import functools

data = """029A
980A
179A
456A
379A
"""

with open("input/2024/day21.txt") as f:
    data = f.read()

sequences = data.strip().split("\n")

keypad = {
    "A": (2, 3),
    "0": (1, 3),
    "1": (0, 2),
    "2": (1, 2),
    "3": (2, 2),
    "4": (0, 1),
    "5": (1, 1),
    "6": (2, 1),
    "7": (0, 0),
    "8": (1, 0),
    "9": (2, 0),
    " ": (0, 3)
}   

dpad = {
    "A": (2, 0),
    "^": (1, 0),       
    "<": (0, 1),       
    "v": (1, 1),       
    ">": (2, 1),       
    " ": (0, 0)
}

def map_buttons(buttons, pad):
    moves = ""

    current_button = "A"
    ex, ey = pad[" "]

    for next_button in buttons:
        current_position = pad[current_button]
        next_position = pad[next_button]

        cx, cy = current_position
        nx, ny = next_position
        dx, dy = nx - cx, ny - cy
        
        ups, downs, lefts, rights = "", "", "", ""

        if dy < 0:
            ups = "^" * abs(dy)
        if dy > 0:
            downs = "v" * dy
        if dx < 0:
            lefts = "<" * abs(dx)
        if dx > 0:
            rights = ">" * dx

        this_move = lefts + downs + ups + rights

        if len(pad) > 6:
            # Move from bottom row to left column
            if cy == ey and nx == ex:
                this_move = ups + lefts + downs + rights
            # Move from left column to bottom row
            if cx == ex and ny == ey:
                this_move = lefts + rights + ups + downs

        if len(pad) <= 6:
            # Move from left column to top row
            if cx == ex and ny == ey:
                this_move = lefts + rights + ups + downs
            # Move from top row to left column
            if cy == ey and nx == ex:
                this_move = ups + downs + lefts + rights
        
        moves += this_move + "A"
        current_button = next_button

    return moves

total = 0

for sequence in sequences:
    buttons = list(sequence)
    step1 = map_buttons(buttons, keypad)
    step2 = map_buttons(step1, dpad)
    step3 = map_buttons(step2, dpad)
    """
    print(sequence)
    print(step1)
    print(step2)
    print(step3)
    print(len(step3))
    break
    """

    #print(len(step3))
    total += len(step3) * int(sequence.replace("A", ""))

print(total)

# part 2
"""
total = 0
for i, sequence in enumerate(sequences):
    print(f"Solving sequence number {i} which is {sequence}")
    buttons = list(sequence)
    next_step = map_buttons(buttons, keypad)
    
    for k in range(25):
        print(f"Currently at keypad number {k+1} / 25 for sequence number {i}")
        next_step = map_buttons(next_step, dpad)

    total += len(next_step) * int(sequence.replace("A", ""))

print(total)
"""

@functools.cache
def map_dpad(start, end, depth):
    if depth == 0:
        return 1

    current_position = dpad[start]
    next_position = dpad[end]

    cx, cy = current_position
    nx, ny = next_position
    dx, dy = nx - cx, ny - cy
    ex, ey = dpad[" "]
    
    ups, downs, lefts, rights = "", "", "", ""

    if dy < 0:
        ups = "^" * abs(dy)
    if dy > 0:
        downs = "v" * dy
    if dx < 0:
        lefts = "<" * abs(dx)
    if dx > 0:
        rights = ">" * dx

    this_move = lefts + downs + ups + rights

    # Move from left column to top row
    if cx == ex and ny == ey:
        this_move = lefts + rights + ups + downs
    # Move from top row to left column
    if cy == ey and nx == ex:
        this_move = ups + downs + lefts + rights

    moves = this_move + "A"

    total_length = 0
    current = start

    #print(f"Mapped {end} to {moves}")
    current = "A"

    for move in moves:
        length = map_dpad(current, move, depth - 1)
        #print(f"Moving {move} was done in {length} moves")
        total_length += length
        current = move

    #print(f"Moving {start} to {end} is {moves}. Has length {total_length} at depth {depth}")

    return total_length


total = 0
for sequence in sequences:
    next_step = map_buttons(sequence, keypad)
    total_length = 0

    #print(f"Step 1 gave {total_length} steps")
    
    current = "A"
    #print(next_step)
    for step in next_step:
        length = map_dpad(current, step, 25)
        #print(f"Mapping '{step}' gave a length of {length} steps")
        total_length += length
        current = step

    #print(f"Sequence: {sequence} | length: {total_length} | Expected 28")
    total += total_length * int(sequence.replace("A", ""))

print(total)

