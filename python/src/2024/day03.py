import re

data = """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"""

with open("input/2024/day03.txt", "r") as f:
    data = f.read()

sum = 0

for line in data.strip().split('\n'):
    matches = re.findall(r"mul\((\d+),(\d+)\)", line)

    for match in matches:
        sum += int(match[0]) * int(match[1])

print(sum)
