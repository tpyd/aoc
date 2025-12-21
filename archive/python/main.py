import sys
import time

year = 2024

if len(sys.argv) > 1:
    day = sys.argv[1]
else:
    day = input("Enter day: ")

with open(f"src/2024/day{int(day):02}.py", "r") as f:
    file = f.read()

start_time = time.time()
exec(file)
stop_time = time.time()

print(f"Execution time: {stop_time - start_time:.4f} seconds")

