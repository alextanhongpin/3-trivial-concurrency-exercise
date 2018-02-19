from concurrent.futures import ThreadPoolExecutor, wait, as_completed
from time import sleep
from random import randint

def delay(msg):
  sleep(randint(3, 5))
  return msg

pool = ThreadPoolExecutor(max_workers = 5)
futures = []
for x in range(5):
  futures.append(pool.submit(delay, 'Hello {}'.format(x)))



# for x in as_completed(futures):
  # print(x.result())

# Options for wait:
# FIRST_COMPLETED
# FIRST_EXCEPTION
# ALL_COMPLETED (default)
for i in wait(futures, return_when = 'ALL_COMPLETED').done:
  print(i.result())