from functools import reduce

num = [x for x in range(1,101)]
square_sum = sum([y*y for y in num])
sum_square = sum(num) * sum(num)

print(sum_square - square_sum) 