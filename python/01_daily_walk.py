from time import sleep
from random import randint
from concurrent.futures import ProcessPoolExecutor



def getting_ready(name):
  print('{} started getting ready'.format(name))
  duration = randint(3, 5)
  sleep(duration)
  print('{} spent {} seconds getting ready'.format(name, duration))

def putting_on_shoes(name):
  print('{} started putting on shoes'.format(name))
  duration = randint(3, 5)
  sleep(duration)
  print('{} spent {} seconds putting on shoes'.format(name, duration))
  return 1

def alarm():
  print('Arming alarm')
  print('Alarm is counting down')
  sleep(6)
  print('Alarm is armed')
  return 0

def main():
  names = ['Alice', 'Bob']
  print('Let\'s go for a walk!')

  with ProcessPoolExecutor(5) as executor:
    pids = [executor.submit(getting_ready, i) for i in names]
    
    for pid in pids:
      pid.result()


  with ProcessPoolExecutor(5) as executor:
    alarm_pool = []
    alarm_pool.append(executor.submit(alarm))


    shoes_pool = [executor.submit(putting_on_shoes, name)
                  for name in names]
    
    count = 0
    for pid in shoes_pool + alarm_pool:
      count += pid.result()
      if count == 2:
        print('Exiting and locking the door.')

if __name__ == '__main__':
  main()