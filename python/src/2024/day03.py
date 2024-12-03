import re

with open("input/2024/day03.txt", "r") as f:
    data = f.read()

sum = 0

matches = re.findall(r"mul\((\d+),(\d+)\)", data)

for match in matches:
    sum += int(match[0]) * int(match[1])

print(sum)

matches = re.findall(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)", data)

sum = 0
enabled = True

for match in matches:
    if match[0] != "":
        enabled = True
    if match[1] != "":
        enabled = False
    if match[2] != "" and enabled:
        sum += int(match[2]) * int(match[3])  

print(sum)
