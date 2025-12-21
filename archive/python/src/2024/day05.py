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

for nums in updates:
    valid = True
    printed = []
    for num in nums:
        nums_before = [r[1] for r in rules if r[0] == num]
        for n in nums_before:
            if n in printed:
                valid = False

        printed.append(num)

    if valid:
        sum += int(nums[len(nums)//2])
    else:
        invalid_updates.append(nums)

print(sum)

sum = 0

for nums in invalid_updates:
    print_order = [nums[0]]
    for num in nums[1:]:
        insert_before = [r[1] for r in rules if r[0] == num and r[1] in print_order]

        if insert_before:
            index = min([print_order.index(n) for n in insert_before])
            print_order.insert(index, num)
        else:
            print_order.append(num)

    sum += int(print_order[len(print_order)//2])

print(sum)
