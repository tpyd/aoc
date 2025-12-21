data = """Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"""

data = """Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"""

with open("input/2024/day17.txt") as f:
    data = f.read()

registers, instructions = data.strip().split("\n\n")

nums = []
for line in registers.split("\n"):
    nums.append(line.split()[-1])

a, b, c = map(int, nums)

instructions_string = instructions.split()[-1]
instructions = list(map(int, instructions_string.split(",")))

def get_value(operand):
    if operand == 4:
        return a
    elif operand == 5:
        return b
    elif operand == 6:
        return c
    else:
        return operand

def set_value(operand, value):
    if operand == 4:
        global a
        a = value
    if operand == 5:
        global b
        b = value
    if operand == 6:
        global c
        c = value

ip = 0
to_print = ""

while ip < len(instructions):
    opcode = instructions[ip]
    operand = instructions[ip+1]
    
    # adv
    if opcode == 0:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(4, new_value)

    # bxl
    if opcode == 1:
        new_value = b ^ operand
        set_value(5, new_value)

    # bst
    if opcode == 2:
        value = get_value(operand)
        new_value = value % 8
        set_value(5, new_value)

    # jnz
    if opcode == 3:
        if a != 0:
            ip = operand
            continue

    # bxc
    if opcode == 4:
        new_value = b ^ c
        set_value(5, new_value)

    # out
    if opcode == 5:
        value = get_value(operand)
        new_value = value % 8
        to_print += str(new_value) + ','

    # bdv
    if opcode == 6:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(5, new_value)

    # cdv
    if opcode == 7:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(6, new_value)
    
    ip += 2

to_print = to_print[:-1]
print(to_print)

# Part 2
# Found answer by converting the a number to an oct, then manually searching
# by adding or subtracting digits. 
# First digit in "a" changes last digit in output.
# Second digit in "a" changes the second last digit in output etc.
# Can easily be automated. Make sure you find the lowest number.
# First part of this code is for manually searching by entering n or -n to change
# digit number n.
# Second part is to verify the number and printing it as decimal number
"""

solution = ",".join([str(i) for i in instructions])
#test_a = 0o6111121200000000
#test_a = 0o6511121000000000
test_a = 0o6562250000000000
while True:
    indata = ""
    while indata == "" or indata == "-":
        indata = input()
    indata = int(indata)
    
    if indata > 0:
        test_a = test_a + 8 ** indata
    else:
        test_a = test_a - 8 ** abs(indata)

    print(test_a)
    a = test_a
    b = 0
    c = 0
    ip = 0
    to_print = ""

    while ip < len(instructions):
        opcode = instructions[ip]
        operand = instructions[ip+1]
        
        # adv
        if opcode == 0:
            value = get_value(operand)
            new_value = a // (2 ** value)
            set_value(4, new_value)

        # bxl
        if opcode == 1:
            new_value = b ^ operand
            set_value(5, new_value)

        # bst
        if opcode == 2:
            value = get_value(operand)
            new_value = value % 8
            set_value(5, new_value)

        # jnz
        if opcode == 3:
            if a != 0:
                ip = operand
                continue

        # bxc
        if opcode == 4:
            new_value = b ^ c
            set_value(5, new_value)

        # out
        if opcode == 5:
            value = get_value(operand)
            new_value = value % 8
            to_print += str(new_value) + ','

        # bdv
        if opcode == 6:
            value = get_value(operand)
            new_value = a // (2 ** value)
            set_value(5, new_value)

        # cdv
        if opcode == 7:
            value = get_value(operand)
            new_value = a // (2 ** value)
            set_value(6, new_value)
        
        ip += 2

    to_print = to_print[:-1]

    mine = len(to_print)
    real = len(solution)
    print(f"{mine}={real} | mine: '{to_print}' | solution: '{solution}' with A: {oct(test_a)}")
    #print("me: " + to_print)
    #print("so: " + solution)
    if to_print == instructions_string:
        break

    #print(to_print)
    #break

print(a)

"""
#a, b, c = map(int, nums)
answer = int(0o6562257414257155)
print(answer)

a = answer
b = 0
c = 0
ip = 0
output = []

while ip < len(instructions):
    opcode = instructions[ip]
    operand = instructions[ip+1]
    
    # adv
    if opcode == 0:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(4, new_value)

    # bxl
    if opcode == 1:
        new_value = b ^ operand
        set_value(5, new_value)

    # bst
    if opcode == 2:
        value = get_value(operand)
        new_value = value % 8
        set_value(5, new_value)

    # jnz
    if opcode == 3:
        if a != 0:
            ip = operand
            continue

    # bxc
    if opcode == 4:
        new_value = b ^ c
        set_value(5, new_value)

    # out
    if opcode == 5:
        value = get_value(operand)
        new_value = value % 8
        output.append(new_value)

    # bdv
    if opcode == 6:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(5, new_value)

    # cdv
    if opcode == 7:
        value = get_value(operand)
        new_value = a // (2 ** value)
        set_value(6, new_value)
    
    ip += 2

print(instructions)
print(output)


