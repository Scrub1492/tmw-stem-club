class Fibonacci:

    cur = None
    next = None

    def __init__(self, first_seed, second_seed):
        pass
    
    def __next__(self):
        pass

def fibonacci(first_seed, second_seed):
    return Fibonacci(first_seed, second_seed)

# Tests
fib = fibonacci(0, 1)
cmp = iter([0, 1, 1, 2, 3, 5, 8, 13, 21, 34])

for x in range(10):
    assert(next(fib) == next(cmp))
