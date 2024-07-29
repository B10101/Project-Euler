from math import prod
from functools import reduce

def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a % b)

llist = [x for x in range(1,21)]
print(reduce(lambda a,b: ((a*b)/gcd(a,b)), llist))


