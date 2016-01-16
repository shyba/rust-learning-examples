from multiprocessing.pool import Pool


def count(number):
    i = 0
    while i < number:
        i += 1

values = [5000000 for _ in range(5)]
with Pool(5) as pool:
    pool.map(count, values)
