data = """
0   3   6   9  12  15
1   3   6  10  15  21
10  13  16  21  30  45
"""

with open("../../input/2023/real/day09.txt") as f:
    data = f.read()

data = data.strip()
lines = data.split('\n')

readydata = []

for line in lines:
    nums = line.split(' ')
    nums = [n for n in nums if n != '']

    readylines = []

    for num in nums:
        linenum = int(num)
        readylines.append(linenum)

    readydata.append(readylines)

# Part 1
def findnext(nums):
    diffs = [right - left for (left, right) in zip(nums[:-1], nums[1:])]

    if len(set(diffs)) == 1:
        diffnum = diffs[0]
    else:
        diffnum = findnext(diffs)

    return nums[-1] + diffnum

newnums = []
for nums in readydata:
    newnumber = findnext(nums) 
    newnums.append(newnumber)

print(sum(newnums))

# Part 2
def findprev(nums):
    diffs = [right - left for (left, right) in zip(nums[:-1], nums[1:])]

    if len(set(diffs)) == 1:
        diffnum = diffs[0]
    else:
        diffnum = findprev(diffs)

    return nums[0] - diffnum

newnums = []
for nums in readydata:
    newnumber = findprev(nums) 
    newnums.append(newnumber)

print(sum(newnums))




