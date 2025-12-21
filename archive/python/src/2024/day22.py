data = """1
10
100
2024
"""

data = """1
2
3
2024
"""

with open("input/2024/day22.txt") as f:
    data = f.read()

nums = map(int, data.strip().split("\n"))
steps = 2000

total = 0

for num in nums:
    for i in range(steps):
        step1 = ((num * 64) ^ num) % 16777216
        step2 = ((step1 // 32) ^ step1) % 16777216
        step3 = ((step2 * 2048) ^ step2) % 16777216
        
        num = step3

    total += num

print(total)

# Part 2
nums = map(int, data.strip().split("\n"))
#nums = [123]
steps = 2000
overview = []

for num in nums:
    prices = [num % 10]

    for i in range(steps - 1):
        step1 = ((num * 64) ^ num) % 16777216
        step2 = ((step1 // 32) ^ step1) % 16777216
        step3 = ((step2 * 2048) ^ step2) % 16777216
        
        num = step3
        prices.append(num % 10)

    diffs = [b - a for a, b in zip(prices[:-1], prices[1:])]
    prices = [(p, d) for p, d in zip(prices[1:], diffs)]

    overview.append(prices)

candidates = []
unique_diffs = set()

for number_overview in overview:
    number_bananas = {}

    for i in range(len(number_overview) - 3):
        block = number_overview[i:i+4]

        diffs = tuple([b[1] for b in block])
        banana = block[-1][0]

        if diffs not in number_bananas:
            number_bananas[diffs] = banana
         
        #if diffs in number_bananas and number_bananas[diffs] < banana:
            #number_bananas[diffs] = banana

        unique_diffs.add(diffs)


    candidates.append(number_bananas)

max_bananas = 0
#print(unique_diffs)

for i, diff in enumerate(unique_diffs):
    print(f"Checking {i} out of {len(unique_diffs)}")
    diff_bananas = 0

    for monkey in candidates:
        if diff in monkey:
            bananas = monkey[diff]
            diff_bananas += bananas

    max_bananas = max(max_bananas, diff_bananas)

# 1755 too high
print(max_bananas)

