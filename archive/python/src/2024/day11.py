data = """125 17"""

with open("input/2024/day11.txt") as f:
    data = f.read()

rocks = [int(d) for d in data.split()]
blinks = 25
#print(rocks)

for i in range(blinks):
    #print(f"Blink number {i+1} / {blinks}")
    new_rocks = []

    for rock in rocks:
        if rock == 0:
            new_rocks.append(1)
            continue
        
        if len(str(rock)) % 2 == 0:
            first = int(str(rock)[:len(str(rock))//2])
            second = int(str(rock)[len(str(rock))//2:])
            new_rocks.append(first)
            new_rocks.append(second)
            continue

        new_rocks.append(rock * 2024)

    rocks = new_rocks

    #print(rocks)

print(len(rocks))

rocks = [int(d) for d in data.split()]
total_blinks = 75
#print(rocks)

import functools

@functools.cache
def calculate(rock, blinks):
    if blinks == 0:
        return 1
    if rock == 0:
        return calculate(1, blinks - 1)  
    elif len(str(rock)) % 2 == 0:
        nums = 0
        first = int(str(rock)[:len(str(rock))//2])
        second = int(str(rock)[len(str(rock))//2:])
        nums += calculate(first, blinks - 1)
        nums += calculate(second, blinks - 1)
        return nums
    else:
        return calculate(rock * 2024, blinks - 1)

sum = 0

for rock in rocks:
    sum += calculate(rock, total_blinks)

print(sum)
