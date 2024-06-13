from math import sqrt

def sieve(n):
    isprime = [True for i in range(n+1)]
    isprime[0] = isprime[1] = False

    for i in range(2, int(sqrt(n) + 1)):
        if isprime[i]:
            for j in range(i*i, n+1, i):
                isprime[j] = False
    prime = [k for k in range(n+1) if isprime[k]]
    return prime

print(sum(sieve(2000000)))
