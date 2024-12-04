with open("input/2024/day04.txt") as f:
    data = f.read()

data = data.strip().split('\n')
data = [list(d) for d in data]

word = "XMAS"
y_max = len(data)
x_max = len(data[0])

occurences = 0

for y in range(y_max):
    for x in range(x_max):
        # Horizontal
        if x + 3 < x_max:
            text = data[y][x] + data[y][x+1] + data[y][x+2] + data[y][x+3]
            occurences += text == word or text[::-1] == word

        # Vertical
        if y + 3 < y_max:
            text = data[y][x] + data[y+1][x] + data[y+2][x] + data[y+3][x]
            occurences += text == word or text[::-1] == word
            
        # Diagonal down-right
        if y + 3 < y_max and x + 3 < x_max:
            text = data[y][x] + data[y+1][x+1] + data[y+2][x+2] + data[y+3][x+3]
            occurences += text == word or text[::-1] == word

        # Diagonal down-left
        if y + 3 < y_max and x - 3 >= 0:
            text = data[y][x] + data[y+1][x-1] + data[y+2][x-2] + data[y+3][x-3]
            occurences += text == word or text[::-1] == word

print(occurences)

occurences = 0
valid_patterns = [
    ('M', 'M', 'S', 'S'),
    ('M', 'S', 'M', 'S'),
    ('S', 'M', 'S', 'M'),
    ('S', 'S', 'M', 'M'),
]

for y in range(1, y_max - 1):
    for x in range(1, x_max - 1):
        if data[y][x] != 'A':
            continue

        letters = (data[y-1][x-1], data[y-1][x+1], data[y+1][x-1], data[y+1][x+1])
        if letters in valid_patterns:
            occurences += 1

print(occurences)
