import math
import sympy

def find_prime_in_range(n):
    lower_bound = n**2
    upper_bound = 2 * n**2
    for num in range(lower_bound + 1, upper_bound):
        if sympy.isprime(num):
            return num
    raise ValueError("No prime found in the given range")

def calculate_m(p, n, epsilon):
    log_p = math.log(p)
    m = (1 + epsilon) * (n + 1) * log_p
    return m

def main(n, epsilon=0.1):
    # Find prime p
    p = find_prime_in_range(n)
    print(f"Prime p: {p}")

    # Calculate m
    m = calculate_m(p, n, epsilon)
    print(f"Calculated m: {m}")

if __name__ == "__main__":
    n = int(input("Security param N: "))
    epsilon = 0.1
    main(n, epsilon)
