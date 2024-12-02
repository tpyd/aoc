with open("input/2024/day01.txt", "r") as f:
    data = f.read()

left_list = []
right_list = []

for line in data.strip().split('\n'):
    nums = line.strip().split()

    left_list.append(int(nums[0]))
    right_list.append(int(nums[1]))

left_list = sorted(left_list)
right_list = sorted(right_list)

diffs = 0

for left, right in zip(left_list, right_list):
    diffs += abs(right - left)

print(diffs)

sums = 0

for left in left_list:
    occurences = right_list.count(left)
    sums += left * occurences

print(sums)
