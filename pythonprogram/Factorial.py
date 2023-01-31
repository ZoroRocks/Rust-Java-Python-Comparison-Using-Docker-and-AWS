import time
from decimal import Decimal

def factorial(n):
    result = Decimal(1)
    for i in range(1, n+1):
        result *= Decimal(i)
    return result

# Example usage


if __name__ == "__main__":
    start_time1 = time.perf_counter_ns()
    factorial(10000)
    print("Python, N = 10000, time:", time.perf_counter_ns() - start_time1)

    start_time2 = time.perf_counter_ns()
    factorial(20000)
    print("Python, N = 20000, time:", time.perf_counter_ns() - start_time2)

    start_time3 = time.perf_counter_ns()
    factorial(30000)
    print("Python, N = 30000, time:", time.perf_counter_ns() - start_time3)