from random import randint, shuffle
from time import sleep
from concurrent.futures import ThreadPoolExecutor, as_completed




def prepare_dishes(menus):
  dishes = []
  for dish in menus:
    dishes += [dish] * randint(5, 10)
  shuffle(dishes)
  return dishes

def eat(person, dish):
  print('{} is enjoying {}'.format(person, dish))
  sleep(randint(3, 5))
  return person

def main():
  people = ['Alice', 'Bob', 'Charlie', 'Dave']
  menus = ['chorizo', 'chopitos', 'pimientos de padron', 'croquetas', 'patatas bravas']
  dishes = prepare_dishes(menus)

  print('Bon Appetit!')
  with ThreadPoolExecutor(max_workers = 4) as executor:
    futures = []
    while len(dishes) > 0:
      while len(people) > 0:
        if len(dishes) == 0:
          break
        dish = dishes.pop()
        person = people.pop()
        futures.append(executor.submit(eat, person, dish))
      for c in as_completed(futures):
        people.append(c.result())
  print('That was delicious!')

if __name__ == '__main__':
  main()