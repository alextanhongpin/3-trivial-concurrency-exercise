extern crate rand;
extern crate threadpool;

use std::collections::HashSet;
use std::sync::mpsc::sync_channel;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use rand::distributions::{IndependentSample, Range};

fn main() {
  let max_count = 25;
  let max_buffer = 8;

  let (tx, rx) = sync_channel(max_buffer);
  let nums: Vec<usize> = (0..max_count).collect();
  let data = Arc::new(Mutex::new(nums));
  let counter = Arc::new(Mutex::new(0));
  let cache = Arc::new(Mutex::new(HashSet::new()));

  let handle = thread::spawn(move || {
    let mut count = 0;
    'outer: loop {
      match rx.recv() {
        Ok(_) => count += 1,
        Err(err) => println!("ReceivedError: {:?}", err),
      }
      if count == max_count {
        break 'outer;
      }
    }
    println!("Done");
  });

  let mut handles = vec![];
  handles.push(handle);

  let between = Range::new(1, 5); // Maximum sleep is 4s
  let mut rng = rand::thread_rng();

  'outer: loop {
    if data.lock().unwrap().is_empty() {
      break 'outer;
    }
    let duration = between.ind_sample(&mut rng);
    let (tx, data, counter, counter_2, cache) = (
      tx.clone(),
      data.clone(),
      counter.clone(),
      counter.clone(),
      cache.clone(),
    );

    let (mut data_raw, mut counter_raw) = (data.lock().unwrap(), counter.lock().unwrap());

    if *counter_raw < 8 {
      match data_raw.pop() {
        Some(val) => {
          *counter_raw += 1;
          let mut cache_raw = cache.lock().unwrap();
          cache_raw.insert(val);
          // Send to thread
          let handle = thread::spawn(move || {
            tx.send(do_work(val, duration)).unwrap();

            let mut counter_raw = counter_2.lock().unwrap();
            *counter_raw -= 1;
          });
          handles.push(handle);
        }
        None => break 'outer,
      }
    } else {
      // push to queue
      let mut cache_raw = cache.lock().unwrap();

      if cache_raw.len() == 8 {
        cache_raw.insert(99);

        'inner: for i in 0..25 {
          if cache_raw.contains(&i) {
            continue 'inner;
          } else {
            println!("Tourist {} is waiting", i);
          }
        }
      }
      thread::yield_now();
      continue 'outer;
    }
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("-----------------------------------------------");
  // Using thread pool
  let thread_pool_counter = Arc::new(Mutex::new(0));
  let thread_pool_cache = Arc::new(Mutex::new(HashSet::new()));
  let pool = ThreadPool::new(8);

  for i in 0..25 {
    let duration = between.ind_sample(&mut rng);
    let (counter, cache) = (thread_pool_counter.clone(), thread_pool_cache.clone());
    let (mut counter_raw, mut cache_raw) = (counter.lock().unwrap(), cache.lock().unwrap());
    if *counter_raw == 8 {
      *counter_raw += 1;
      'inner_thread: for i in 0..25 {
        if cache_raw.contains(&i) {
          continue 'inner_thread;
        } else {
          println!("*Tourist {} is waiting", i);
        }
      }
    } else if *counter_raw < 8 {
      cache_raw.insert(i);
      *counter_raw += 1;
    }
    pool.execute(move || {
      println!("*Tourists {} is online", i);
      thread::sleep(time::Duration::from_secs(duration));
      println!("*Tourists {} is done after {} hours", i, duration);
    });
  }
  thread::sleep(time::Duration::from_secs(100));
}

fn do_work(i: usize, duration: u64) -> usize {
  println!("Tourists {} is online", i);
  thread::sleep(time::Duration::from_secs(duration));
  println!("Tourists {} is done after {} hours", i, duration);
  i
}
