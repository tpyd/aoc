import re

test1 = """
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""

with open("../../input/2023/real/day05.txt") as f:
    real = f.read()

data = real

groups = data.strip().split('\n\n')
seeds = groups[0]
groups = groups[1:]

seeds = seeds.strip().split(':')[1].strip().split(' ')
seeds = [int(s) for s in seeds]

processed = []
    
for seed in seeds:
    newseed = seed
    for group in groups:
        maps = group.split(':')[1].strip().split('\n')
        
        transformed = False
        for smap in maps:
            dest, src, rng = smap.split(' ')
            dest = int(dest)
            src = int(src)
            rng = int(rng)
            if newseed >= src and newseed < src + rng:
                newseed = newseed - src + dest
                break

    processed.append(newseed)

print(min(processed))


# Part 2 
segments = data.split('\n\n')
intervals = []

for seed in re.findall(r'(\d+) (\d+)', segments[0]):
    x1, dx = map(int, seed)
    x2 = x1 + dx
    intervals.append((x1, x2, 1))

min_location = float('inf')
while intervals:
    x1, x2, level = intervals.pop()

    if level == 8:
        min_location = min(x1, min_location)
        continue

    for conversion in re.findall(r'(\d+) (\d+) (\d+)', segments[level]):
        z, y1, dy = map(int, conversion)
        y2 = y1 + dy
        diff = z - y1

        # No overlap
        if x2 <= y1 or x1 >= y2:
            continue

        # Overlap on the left side
        if x1 < y1:
            intervals.append((x1, y1, level))
            x1 = y1

        # Overlap on the right side
        if x2 > y2:
            intervals.append((y2, x2, level))
            x2 = y2

        # Pass to next level
        intervals.append((x1 + diff, x2 + diff, level + 1))
        break

    else:
        intervals.append((x1, x2, level + 1))

print(min_location)



"""
groups = [g.split(':')[1].strip().split('\n') for g in groups]
steps = [[[int(a) for a in n.split(' ')] for n in g] for g in groups]

for mappings in steps:
    print(f"Current seed ranges: {seedranges}")
    newseeds = []
    for seed in seedranges:
        print(f"Seed: {seed}")

        mappedranges = []
        for mapping in mappings:
            print(f"Going to use seed {seed} and map the values {mapping}")
            
            # Transform is all within the seedrange
            # T    `---`
            # S `---------`
            if mapping[1] >= seed[0] and mapping[1] + mapping[2] <= seed[1]:
                print("Mapping is within the seed range")
                mappedranges.append((mapping[1], mapping[1] + mapping[2], mapping[0] - mapping[1]))
                continue

            # Transform takes the entire seedrange
            # T `---------`
            # S    `---`
            if mapping[1] <= seed[0] and mapping[1] + mapping[2] >= seed[1]:
                print("Mapping will map the entire range")
                mappedranges.append((seed[0], seed[1], mapping[0] - mapping[1]))
                break

            # Transform snips on the left side
            # T  `-------`
            # S      `-------`
            if mapping[1] < seed[0] and mapping[1] + mapping[2] > seed[0]:
                print("Mapping is covering the left side of the seed range")
                mappedranges.append((seed[0], mapping[1] + mapping[2], mapping[0] - mapping[1]))
                continue

            # Transform snips on the right side
            # T     `------`
            # S `-------`
            if mapping[1] >= seed[0] and mapping[1] < seed[1] and mapping[1] + mapping[2] > seed[1]:
                print("Mapping is covering the right side of the seed range")
                mappedranges.append((mapping[1], seed[1], mapping[0] - mapping[1]))
                continue

            # Transform totally misses
            # T         `-----`
            # S  `----`
            # or
            # T `-----`
            # S          `----`
            if mapping[1] >= seed[1] or mapping[1] + mapping[2] <= seed[0]:
                print("Mapping will not cover anything")
                # Useless, can just skip this i think
                continue

            
        # Create new ranges that are not mapped
        print(f"Now you need to create ranges from {seed} intersecting with {mappedranges}")

        start = min(seed[0], ) 

        print(mappedranges)
        mappedranges = sorted(mappedranges, key=lambda x: x[0])
        print(mappedranges)

        

    seedranges = newseeds
        
            

# Part 2 old garbage

# One step, for example: water-to-light
currentrangesleft = seedranges
transformedthisstep = []
for si, step in enumerate(steps):
    print(f"STEP: {si+1}")
    print(f"\tCURRENTRANGES: {currentrangesleft}")

    # One mappings in a step, for example: 88 18 7
    for ti, transform in enumerate(step):
        print(f"TRANSFORM: {ti+1} - Current transform: {transform}")

        # All seedranges that havent been mapped yet
        counter = 0
        transformedranges = []

        while True:
            print(counter)
            try:
                seedrange = currentrangesleft[counter]
            except:
                break

            print(transform, seedrange)

            # All possible snips that snips a range

            # Transform is all within the seedrange
            # T    `---`
            # S `---------`
            if transform[1] >= seedrange[0] and transform[1] + transform[2] <= seedrange[1]:
                print("\tTransform in the middle of the seedrange")
                newrange = (transform[1] - transform[1] + transform[0], transform[1] + transform[2] - transform[1] + transform[0])
                oldranges = []
                if seedrange[0] != transform[1]:
                    oldranges.append((seedrange[0], transform[1]))
                else:
                    print("\tStart range is 0 length, skipping it")
                if transform[1] + transform[2] == seedrange[1]:
                    oldranges.append((transform[1] + transform[2], seedrange[1]))
                else:
                    print("\tEnd range is 0 length, skipping it")

                print(f"New range: {newrange} created from {seedrange} - Unchanged ranges: {oldranges}")

                transformedranges.append(newrange)
                transformedthisstep.append(newrange)
                if len(oldranges) > 0:
                    currentrangesleft.extend(oldranges)
                counter = 0
                continue

            # Transform takes the entire seedrange
            # T `---------`
            # S    `---`
            if transform[1] <= seedrange[0] and transform[1] + transform[2] >= seedrange[1]:
                print("Transform bigger than seedrange")
                newrange = (seedrange[0] - transform[1] + transform[0], seedrange[1] - transform[1] + transform[0])
                print(f"New range: {newrange} created from {seedrange}")
                transformedranges.append(newrange)
                transformedthisstep.append(newrange)
                counter = 0
                continue

            # Transform snips on the left side
            # T  `-------`
            # S      `-------`
            if transform[1] < seedrange[0] and transform[1] + transform[2] > seedrange[0]:
                print("Transform snips on the left side")
                newrange = (seedrange[0] - transform[1] + transform[0], transform[1] + transform[2] - transform[1] + transform[0])
                oldrange = (transform[1] + transform[2], seedrange[1])
                print(f"New range: {newrange} created from {seedrange} - Old range: {oldrange}")

                transformedranges.append(newrange)
                transformedthisstep.append(newrange)
                currentrangesleft.append(oldrange)
                counter = 0
                continue

            # Transform snips on the right side
            # T     `------`
            # S `-------`
            if transform[1] >= seedrange[0] and transform[1] < seedrange[1] and transform[1] + transform[2] > seedrange[1]:
                print("Transform snips on the right side")
                newrange = (transform[1] - transform[1] + transform[0], seedrange[1] - transform[1] + transform[0])
                oldrange = (seedrange[0], transform[1])
                print(f"New range: {newrange} created from {seedrange} - Old range: {oldrange}")

                transformedranges.append(newrange)
                transformedthisstep.append(newrange)
                currentrangesleft.append(oldrange)
                counter = 0
                continue

            # Transform totally misses
            # T         `-----`
            # S  `----`
            # or
            # T `-----`
            # S          `----`
            if transform[1] >= seedrange[1] or transform[1] + transform[2] <= seedrange[0]:
                print("Totally miss")
                counter += 1
                if counter > len(currentrangesleft):
                    break
                if seedrange not in currentrangesleft:
                    currentrangesleft.append(seedrange)
                continue

            print("No case hit")
 
        print(f"All currentranges done for this transform, moving to next transform ({ti+2})")
    currentrangesleft.extend(transformedranges)

    print(f"All transforms done in this step, moving to next step ({si+2})")
    print()
    print(currentrangesleft)

    if si >= 9:
        import sys
        sys.exit(1)

print(currentrangesleft)
min_location = min([r[0] for r in currentrangesleft])
print(min_location)


"""
