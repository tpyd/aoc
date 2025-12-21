import itertools

with open("input/2024/day07.txt") as f:
    data = f.read()

data = data.strip().split("\n")
calibration_result = 0

for row in data:
    left, right = row.split(":")
    value = int(left)
    numbers = right.split()

    operator_slots = len(numbers) - 1

    for operators in itertools.product(["+", "*"], repeat=operator_slots):
        result = numbers[0]
        for operator, number in zip(operators, numbers[1:]):
            result = eval(f"{result} {operator} {number}")

            if result > value:
                break

        if result == value:
            calibration_result += value 
            break

print(calibration_result)

calibration_result = 0

for row in data:
    left, right = row.split(":")
    value = int(left)
    numbers = right.split()

    num_operators = len(numbers) - 1

    for operators in itertools.product(["+", "*", "||"], repeat=num_operators):
        result = int(numbers[0])
        for operator, number in zip(operators, numbers[1:]):
            pre_result = result
            if operator == "+":
                result += int(number)
            elif operator == "*":
                result *= int(number)
            elif operator == "||":
                result = result * 10 ** len(number) + int(number)

            if result > value:
                break

        if result == value:
            calibration_result += value 
            break

print(calibration_result)
