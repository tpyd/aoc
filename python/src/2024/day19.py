import functools

data = """r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"""

with open("input/2024/day19.txt") as f:
    data = f.read()

towels, designs = data.strip().split("\n\n")

towels = [t.strip() for t in towels.strip().split(",")]
designs = designs.split("\n")


def check_design(design):
    if design == "":
        return True

    for towel in towels:
        towel_length = len(towel)
        
        if design[:towel_length] == towel:
            possible = check_design(design[towel_length:])
            
            if possible:
                return True


num_possible = 0

for design in designs:
    possible = check_design(design)
    if possible:
        num_possible += 1

print(num_possible)


# Part 2
@functools.cache
def check_design2(design):
    if design == "":
        return 1

    num_possible_designs = 0
    towels_to_check = [t for t in towels if design.startswith(t)]

    for towel in towels_to_check:
        towel_length = len(towel)
        
        if design[:towel_length] == towel:
            subdesigns = check_design2(design[towel_length:])
            num_possible_designs += subdesigns

    return num_possible_designs


num_possible = 0

for design in designs:
    combinations = check_design2(design)
    num_possible += combinations

print(num_possible)
