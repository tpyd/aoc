with open("input/2024/day02.txt", "r") as f:
    data = f.read()

num_safe = 0

for line in data.strip().split('\n'):
    nums = [int(n) for n in line.split()]
    diffs = [nums[i+1] - nums[i] for i in range(len(nums) - 1)]
    ordered = [d > 0 for d in diffs]

    increasing = all([d > 0 for d in diffs])
    decreasing = all([d < 0 for d in diffs])
    safe = all([1 <= abs(d) <= 3 for d in diffs])

    if (increasing or decreasing) and safe:
        num_safe += 1

print(num_safe)

num_safe_2 = 0

for line in data.strip().split('\n'):
    nums = [int(n) for n in line.split()]
    diffs = [nums[i+1] - nums[i] for i in range(len(nums) - 1)]
    ordered = [d > 0 for d in diffs]

    increasing = all([d > 0 for d in diffs])
    decreasing = all([d < 0 for d in diffs])
    safe = all([1 <= abs(d) <= 3 for d in diffs])

    if (increasing or decreasing) and safe:
        num_safe_2 += 1
        continue

    for i in range(len(nums)):
        new_nums = nums[:i] + nums[i+1:]
        diffs = [new_nums[i+1] - new_nums[i] for i in range(len(new_nums) - 1)]
        ordered = [d > 0 for d in diffs]

        increasing = all([d > 0 for d in diffs])
        decreasing = all([d < 0 for d in diffs])
        safe = all([1 <= abs(d) <= 3 for d in diffs])

        if (increasing or decreasing) and safe:
            num_safe_2 += 1
            break

print(num_safe_2)

