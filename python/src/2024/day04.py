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
        # Horizontal right
        if x + 3 < x_max:
            text = data[y][x] + data[y][x+1] + data[y][x+2] + data[y][x+3]
            if text == word:
                occurences += 1

        # Horizontal left
        if x - 3 >= 0:
            text = data[y][x] + data[y][x-1] + data[y][x-2] + data[y][x-3]
            if text == word:
                occurences += 1

        # Vertical down
        if y + 3 < y_max:
            text = data[y][x] + data[y+1][x] + data[y+2][x] + data[y+3][x]
            if text == word:
                occurences += 1
            
        # Vertical up
        if y - 3 >= 0:
            text = data[y][x] + data[y-1][x] + data[y-2][x] + data[y-3][x]
            if text == word:
                occurences += 1

        # Diagonal down-right
        if y + 3 < y_max and x + 3 < x_max:
            text = data[y][x] + data[y+1][x+1] + data[y+2][x+2] + data[y+3][x+3]
            if text == word:
                occurences += 1

        # Diagonal up-left
        if y - 3 >= 0 and x - 3 >= 0:
            text = data[y][x] + data[y-1][x-1] + data[y-2][x-2] + data[y-3][x-3]
            if text == word:
                occurences += 1

        # Diagonal down-left
        if y + 3 < y_max and x - 3 >= 0:
            text = data[y][x] + data[y+1][x-1] + data[y+2][x-2] + data[y+3][x-3]
            if text == word:
                occurences += 1

        # Diagonal up-right
        if y - 3 >= 0 and x + 3 < x_max:
            text = data[y][x] + data[y-1][x+1] + data[y-2][x+2] + data[y-3][x+3]
            if text == word:
                occurences += 1
        
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
