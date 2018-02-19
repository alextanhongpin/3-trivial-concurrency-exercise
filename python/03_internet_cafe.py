from time import sleep
from concurrent.futures import ThreadPoolExecutor, as_completed
from random import randint

def go_online(i):
  print('Tourist {} is online'.format(i))
  duration = randint(3, 5)
  sleep(duration)
  print('Tourist {} spend {} hours online'.format(i, duration))
  return True

def queue(i):
  print('Tourist {} is waiting to go online'.format(i))

def check_queue(tourists, already_online):
  [queue(tourist) for tourist in tourists if tourist not in already_online]

def main():
  already_online = {}
  tourists = [i for i in range(25)]
  is_queue = False
  with ThreadPoolExecutor(max_workers = 8) as executor:
    # futures = executor.map(go_online, range(25))
    while len(tourists) != 0:
      while len(already_online.values()) < 8:
        if len(tourists) == 0:
          break
        tourist = tourists.pop()
        already_online[executor.submit(go_online, tourist)] = tourist
        if len(already_online.values()) == 8 and not is_queue:
          check_queue(tourists, already_online.values())
          is_queue = True
      for future in as_completed(already_online):        
        future.result()
        del already_online[future]
        if len(tourists) == 0:
          break
        tourist = tourists.pop()
        already_online[executor.submit(go_online, tourist)] = tourist



if __name__ == '__main__':
  main()