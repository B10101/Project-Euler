def prime_factors(n):
    factors = []
    
    # Handle the factor of 2
    while n % 2 == 0:
        factors.append(2)
        n //= 2

    # Handle the factor of 3
    while n % 3 == 0:
        factors.append(3)
        n //= 3

    # Check for odd factors from 5 onwards
    factor = 5
    while factor * factor <= n:
        while n % factor == 0:
            factors.append(factor)
            n //= factor
        factor += 2
        while n % factor == 0:
            factors.append(factor)
            n //= factor
        factor += 4  # Skip even numbers after 5

    # If n is still greater than 1, then it must be a prime factor
    if n > 1:
        factors.append(n)
    
    return factors

# Example usage:
number = 315
print(f"Prime factors of {number}: {prime_factors(number)}")
