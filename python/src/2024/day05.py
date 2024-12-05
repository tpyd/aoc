with open("input/2024/day05.txt") as f:
    data = f.read()

raw_rules, raw_updates = data.strip().split('\n\n')

rules = []

for line in raw_rules.split('\n'):
    before, after = line.split('|')
    rules.append((before, after))

updates = [u.split(',') for u in raw_updates.split('\n')]
invalid_updates = []

sum = 0

def is_valid(nums):
    printed = []
    for num in nums:
        nums_before = [r[1] for r in rules if r[0] == num]
        for n in nums_before:
            if n in printed:
                return False

        printed.append(num)

    return True

for nums in updates:
    if is_valid(nums):
        sum += int(nums[len(nums)//2])
    else:
        invalid_updates.append(nums)

print(sum)

sum = 0

for nums in invalid_updates:
    print_order = [nums[0]]
    for num in nums[1:]:
        put_before = [r[1] for r in rules if r[0] == num]
        put_before = [n for n in put_before if n in nums]
        indicies = []
        for n in put_before:
            if n in print_order:
                indicies.append(print_order.index(n))
        if indicies:
            min_index = min(indicies)
            print_order.insert(min_index, num)
        else:
            print_order.append(num)

    sum += int(print_order[len(print_order)//2])

print(sum)
