from math import sqrt, log

def sieve(n):
    isPrime = [True] * (n+1)
    isPrime[0] = isPrime[1] = False
    for p in range(2, int(sqrt(n) + 1)):
        if isPrime[p]:
            for multiple in range(p*p, n + 1, p):
                isPrime[multiple] = False

    primes = [p for p in range(n+1) if isPrime[p]]
    return primes

def nth(f):
    if f == 1:
        return 2
    upper_bound = int((log(f)) + (f*log(log(f))))
    primes = []

    while len(primes) < f:
        primes = sieve(upper_bound)
        upper_bound *= 2

    return primes[f-1]

print(nth(10001))


