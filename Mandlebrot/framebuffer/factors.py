def factors(n):
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return (i, n // i)
    return None

print(factors(14))