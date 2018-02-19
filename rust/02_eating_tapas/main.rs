extern crate rand;

use rand::{Rng};
use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap};
use std::{thread, time};
use std::sync::mpsc::sync_channel;

fn main() {

  let (tx, rx) = sync_channel(4);
  let between = Range::new(3, 5);
          
  let menu = vec!["chorizo", "chopitos", "pimientos de padron", "croquetas", "patatas bravas"];
  let mut dishes = make_dish(&menu);
  let people = vec!["Alice", "Bob", "Charlie", "Dave"];

  let mut cache = HashMap::new();
  cache.insert("Alice", false);
  cache.insert("Bob", false);
  cache.insert("Charlie", false);
  cache.insert("Dave", false);

  println!("Bon Appetit!");
  loop {
    'inner: for person in people.clone() {
      match cache.get(&person) {
        Some(&false) => {
          match dishes.pop() {
            Some(dish) => {
              let tx = tx.clone();
              println!("{} is enjoying some {}", person, dish);
              *cache.get_mut(person).unwrap() = true;
              thread::spawn(move|| {
                let mut rng = rand::thread_rng();
                let duration = between.ind_sample(&mut rng);
                let delay = time::Duration::from_secs(duration);
                thread::sleep(delay);
                tx.send(person).unwrap();
              });
            },
            _ => break,
          }
        },
        Some(&true) => continue 'inner,
        _ => break 'inner
      }
    }

    let j = rx.recv().unwrap();
    *cache.get_mut(j).unwrap() = false;

    if dishes.len() == 0 {
      break;
    }
  }
  println!("That was delicious!");
}

fn make_dish<'a>(menu: &Vec<&'a str>) ->  Vec<&'a str> {
  let mut dishes = vec![];

  let between = Range::new(5, 10);
  let mut rng = rand::thread_rng();

  for dish in menu.clone() {
    let val = between.ind_sample(&mut rng);
    for _ in 1..val {
      dishes.push(dish);
    };
  };

  rng.shuffle(&mut dishes);

  dishes
}