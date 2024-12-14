import re

with open("input/2024/day13.txt") as f:
    data = f.read()

data = data.strip().split("\n\n")
tokens_part1 = 0
tokens_part2 = 0

for block in data:
    button_a, button_b, prize = block.split("\n")

    regex = r"(\d+), Y[+=](\d+)"
    a, b = map(int, re.findall(regex, button_a)[0])
    c, d = map(int, re.findall(regex, button_b)[0])
    m, n = map(int, re.findall(regex, prize)[0])

    m2 = m + 10000000000000
    n2 = n + 10000000000000

    z = a * d - b * c
    x = (m * d - n * c) / z
    y = (n * a - m * b) / z
        
    x2 = (m2 * d - n2 * c) / z
    y2 = (n2 * a - m2 * b) / z

    if x2 % 1 == 0 and y2 % 1 == 0:
        tokens_part2 += x2 * 3 + y2

    if x % 1 == 0 and y % 1 == 0 and x <= 100 and y <= 100:
        tokens_part1 += x * 3 + y
    
print(int(tokens_part1))
print(int(tokens_part2))

