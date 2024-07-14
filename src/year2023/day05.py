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
                #print(f"newseed: {newseed}, dest: {dest}, src: {src}, rng: {rng}")
                newseed = newseed - src + dest
                break

        #print(newseed)

    processed.append(newseed)

print(min(processed))

