import random
import time

LENGTH = 7
SORT_TIMES = 1000
arr = [x for x in range(0, LENGTH)]

total_time = 0
start_time = time.time()
iterations = 0

for i in range(SORT_TIMES):
	sorted = False
	random.shuffle(arr)
	while not sorted:
		iterations += 1
		count = 0
		for e in arr:
			# are the numbers sequential?
			if e == count:
				count += 1
				if count == LENGTH:
					sorted = True
					break
			# if not, just shuffle the entire array and try again!
			else:
				random.shuffle(arr)
				break

end_time = time.time() - start_time

print(f"\nPython Bogosort")
print(f"Bogosort completed for array length {LENGTH}")
print(f"Bogosort performed {SORT_TIMES} times");
print(f"{iterations} iterations performed")
print(f"It took {end_time} seconds")
print(f"An average of {end_time / SORT_TIMES} seconds per sort")