data = """2333133121414131402"""

with open("input/2024/day09.txt") as f:
    data = f.read()

data = data.strip()

layout = []

for i, num in enumerate(data):
    if i % 2 == 0:
        layout.append((int(num), i // 2))
    else:
        layout.append((int(num), '.'))

front = 0
back = len(layout) - 1

while front <= back:
    # Find first available front
    while layout[front][1] != '.':
        front += 1

    # Find first number back
    while layout[back][1] == '.':
        back -= 1

    if back - front == 1:
        layout.pop(front)
        break

    # Remove the blocks
    back_block = layout.pop(back) 
    front_block = layout.pop(front)

    # Create new block to insert
    if front_block[0] > back_block[0]:
        # Fill part the front block, remove back block
        new_block = (back_block[0], back_block[1])
        new_front = (front_block[0] - back_block[0], '.')

        layout.insert(front, new_block)
        layout.insert(front + 1, new_front)
        
        # Add to the empty block in the back
        #layout[-1] = (layout[-1][0] + back_block[0], '.')
    elif front_block[0] == back_block[0]:
        # Replace both front and back block
        layout.insert(front, back_block)
        #layout[-1] = (layout[-1][0] + back_block[0], '.')
    else:
        # Fill entire front block, and reduce back block
        new_front = (front_block[0], back_block[1])
        new_back = (back_block[0] - front_block[0], back_block[1])
        layout.insert(front, new_front)
        layout.insert(back, new_back)

        # Update empty block
        #layout[-1] = (layout[-1][0] + front_block[0], '.')
        
layout = [block for block in layout if block[1] != '.']

counter = 0
checksum = 0

for chunk in layout:
    for _ in range(chunk[0]):
        checksum += counter * chunk[1]
        counter += 1

print(checksum)

#data = """2333133121414131499"""
layout = []

for i, num in enumerate(data):
    if i % 2 == 0:
        layout.append((int(num), i // 2))
    else:
        layout.append((int(num), '.'))

#print(layout)

back = len(layout)
while True:
    #print("------- GONNA PRINT ---------")
    #print(layout)

    # Find first number back
    back -= 1
    while layout[back][1] == '.':# and layout[back][2]:
        back -= 1

    # Find first available front
    front = 0
    for i in range(back):
        #print(f"searching where to place {layout[back]}... trying {layout[i]}")
        if layout[i][1] != '.':
            #print("Its not empty")
            continue

        elif layout[i][0] < layout[back][0]:
            #print("It doesnt have enough space")
            continue

        elif i >= back:
            #print("We've gone too far")
            break

        else:
            #print(f"Found the perfect match: {layout[i]}")
            front = i
            break

    if back < 1:
        break

    if front == 0:
        #print("This is the end")
        continue

    #while layout[front][1] != '.' or layout[front][0] < layout[back][0] and back < front:
        #front += 1
    #print(layout[front], layout[back])

    #print(back - front)
    if back <= front:
        layout.pop(front)
        break

    # Remove the blocks
    back_block = layout.pop(back) 
    front_block = layout.pop(front)

    #print(front_block, back_block)

    # Create new block to insert
    if front_block[0] > back_block[0]:
        #print("Filling part of the front block")

        # Move the block and leave empty space
        new_block = (back_block[0], back_block[1])
        new_front = (front_block[0] - back_block[0], '.')

        layout.insert(front, new_block)
        layout.insert(front + 1, new_front)
        
        # Change the back block to be empty, always accompanied with an empty block
        #new_back = layout.pop(back)
        new_back = (back_block[0], '.')
        layout.insert(back, new_back)
        moved = True
    elif front_block[0] == back_block[0]:
        #print("Front block fits perfectly")
        # Replace both front and back block
        new_front = (back_block[0], back_block[1])
        layout.insert(front, new_front)
        layout.insert(back, front_block)
        moved = True
        #layout[-1] = (layout[-1][0] + back_block[0], '.')
    else:
        # Do nothing
        #print("Cant move this block")
        layout.insert(front, front_block)
        layout.insert(back, back_block)
        #print(layout)

    #print(layout)

        
counter = 0
checksum = 0

for chunk in layout:
    for _ in range(chunk[0]):
        if chunk[1] != '.':
            checksum += counter * chunk[1]
        counter += 1

print(checksum)

