from concurrent.futures import ProcessPoolExecutor
from time import sleep

def sleep_five_seconds(msg):
  sleep(5)
  return msg

# Use process pool for CPU intensive tasks
pool = ProcessPoolExecutor(max_workers = 5)
future = pool.submit(sleep_five_seconds, 'hello')

print(future.done())
sleep(6)
print(future.done())
print(future.result())
