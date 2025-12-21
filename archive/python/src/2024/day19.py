import functools

with open("input/2024/day19.txt") as f:
    data = f.read()

towels, designs = data.strip().split("\n\n")

towels = [t.strip() for t in towels.strip().split(",")]
designs = designs.split("\n")

@functools.cache
def check_design(design):
    if design == "":
        return 1

    num_possible_designs = 0
    towels_to_check = [t for t in towels if design.startswith(t)]

    for towel in towels_to_check:
        towel_length = len(towel)
        
        if design[:towel_length] == towel:
            subdesigns = check_design(design[towel_length:])
            num_possible_designs += subdesigns

    return num_possible_designs


num_possible = 0
possible_designs = 0

for design in designs:
    combinations = check_design(design)
    possible_designs += combinations

    if combinations > 0:
        num_possible += 1

print(num_possible)
print(possible_designs)
