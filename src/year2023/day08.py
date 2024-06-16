import math

with open("../../input/2023/real/day08.txt") as f:
    real = f.read()


def part1():
    test1 = """
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    """

    test2 = """
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    """

    data = real

    instructions, map_data_str = data.split('\n\n')
    instructions = instructions.strip()
    map_data_str = map_data_str.strip()

    map_data = {}
    for split in map_data_str.split('\n'):
        current, directions = split.split('=')
        current = current.strip()
        directions = directions.strip()
        directions = directions.replace('(', '')
        directions = directions.replace(')', '')
        directions = directions.replace(' ', '')
        left, right = directions.split(',')
        
        map_data[current] = (left, right)

    current_instruction = 0
    current_location = "AAA"
    steps = 0
    while True:
        direction = instructions[current_instruction]
        current_instruction += 1
        if current_instruction + 1 > len(instructions):
            current_instruction = 0

        directions = map_data[current_location]
        left, right = directions
        if direction == "L":
            current_location = left
        else:
            current_location = right

        steps += 1
        
        if current_location == "ZZZ":
            break

    print(f"Part 1: {steps}")


def part2():
    test = """
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    """

    data = real

    instructions, map_data_str = data.split('\n\n')
    instructions = instructions.strip()
    map_data_str = map_data_str.strip()

    map_data = {}
    for split in map_data_str.split('\n'):
        current, directions = split.split('=')
        current = current.strip()
        directions = directions.strip()
        directions = directions.replace('(', '')
        directions = directions.replace(')', '')
        directions = directions.replace(' ', '')
        left, right = directions.split(',')
        
        map_data[current] = (left, right)

    current_locations = [l for l in map_data.keys() if l[2] == "A"]
    steps = 0
    current_instruction = 0
    loops = []

    for current_location in current_locations:
        current_instruction = 0
        location = current_location
        steps = 0
        first_hit = False

        while True:
            direction = instructions[current_instruction]
            current_instruction += 1
            if current_instruction + 1 > len(instructions):
                current_instruction = 0

            directions = map_data[location]
            left, right = directions
            if direction == "L":
                location = left
            else:
                location = right

            steps += 1

            if location[2] == "Z":
                if not first_hit:
                    steps_to_first = steps
                    first_hit = True
                elif first_hit:
                    steps_to_second = steps
                    break

        loops.append(steps_to_first)
    
    ans = math.lcm(*loops)

    print(f"Part 2: {ans}")

part1()
part2()

