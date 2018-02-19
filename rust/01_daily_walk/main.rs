extern crate rand;

use std::{thread, time};
use rand::distributions::{IndependentSample, Range};

fn main() {
  let between = Range::new(1, 2);
  let names = vec!["Alice", "Bob"];
  let mut handles = vec![];

  for name in names.clone() {
    let handle = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let duration = between.ind_sample(&mut rng);
        getting_ready(name, duration);
    });
    handles.push(handle);
  }

  println!("Let's go for a walk!");
  for handle in handles {
    handle.join().unwrap();
  }

  handles = vec![];
  let mut names_clone = names.clone();
  names_clone.extend(vec!["alarm"]);

  for name in names_clone {
    let handle = thread::spawn(move || {
      match name {
        "alarm" => {
          alarm(4);
        },
        _ => {
          let mut rng = rand::thread_rng();
          let duration = between.ind_sample(&mut rng);
          putting_on_shoes(name, duration);
        }
      }

    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}

fn getting_ready(name: &'static str, duration: u64) {
  println!("{} started getting ready", name);
  let delay = time::Duration::from_secs(duration);
  thread::sleep(delay);
  println!("{} spent {:?} seconds getting ready", name, delay.as_secs());
}


fn putting_on_shoes(name: &'static str, duration: u64) {
	println!("{} started putting on shoes", name);
  let delay = time::Duration::from_secs(duration);
  thread::sleep(delay);
  println!("{} spend {} seconds putting on shoe", name, delay.as_secs());
}

fn alarm(duration: u64) {
  	println!("Arming alarm");
    println!("Alarm is counting down...");
    let delay = time::Duration::from_secs(duration);
    thread::sleep(delay);
  	println!("Alarm is armed");
}