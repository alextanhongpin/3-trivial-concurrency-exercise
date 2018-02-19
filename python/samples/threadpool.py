from concurrent.futures import ThreadPoolExecutor
from time import sleep

def sleep_five_seconds(msg):
  sleep(5)
  return msg

# Use thread pool for network operations or I/O
pool = ThreadPoolExecutor(max_workers = 5)
future = pool.submit(sleep_five_seconds, 'hello')

print(future.done())
sleep(5)
print(future.done())
print(future.result())
