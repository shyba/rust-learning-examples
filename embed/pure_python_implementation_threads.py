from multiprocessing.pool import ThreadPool

pool = ThreadPool(5)

def count(number):
    i = 0
    while i < number:
        i += 1

values = [5000000 for _ in range(5)]
pool.map(count, values)
