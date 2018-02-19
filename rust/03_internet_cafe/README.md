

Attempts to parallelize this works:

```rust
extern crate rand;

use std::sync::mpsc::sync_channel;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use rand::distributions::{IndependentSample, Range};

fn main() {
  let max_count = 25;
  let max_buffer = 5;

  let (tx, rx) = sync_channel(max_buffer);
  let nums: Vec<usize> = (0..max_count).collect();
  let data = Arc::new(Mutex::new(nums));

  let between = Range::new(1, 5);
  let mut rng = rand::thread_rng();

  thread::spawn(move || {
    let mut count = 0;
    'outer: loop {
      match rx.recv() {
        Ok(val) => {
          println!("Received: {:?}", val);
          count += 1;
        },
        Err(err) => println!("ReceivedError: {:?}", err),
      }
      if count == max_count {
        break 'outer;
      }
    }
    println!("Done");
  });

  let mut handles = vec![];

  // while values.lock().unwrap().len() != 0 {
  'outer: for i in 0..5 {
    println!("start");

    let duration = between.ind_sample(&mut rng);
    let tx_clone = tx.clone();
    let data = data.clone();
  
    let handle = thread::spawn(move || {
      println!("Spawned thread {}", i);
      for j in 0..5 {
        tx_clone.send(do_work(i + j, duration, i));
      };
    });

    handles.push(handle);
  }

  // For the sake of synchronization
  for handle in handles {
    handle.join().unwrap();
  }
}

fn do_work(i: usize, duration: u64, thread: usize) -> usize {
  println!("Thread {} sleep for {} seconds: {}", thread, duration, i);
  thread::sleep(time::Duration::from_secs(duration));
  i
}
```

Output:

```
start
start
spawned thread 0
Thread 0 sleep for 1 seconds: 0
start
start
spawned thread 1
Thread 1 sleep for 1 seconds: 1
start
spawned thread 3
Thread 3 sleep for 2 seconds: 3
spawned thread 2
Thread 2 sleep for 1 seconds: 2
spawned thread 4
Thread 4 sleep for 4 seconds: 4
Thread 1 sleep for 1 seconds: 2
Thread 2 sleep for 1 seconds: 3
Thread 0 sleep for 1 seconds: 1
Received: 0
Received: 1
Received: 2
Thread 3 sleep for 2 seconds: 4
Received: 3
Thread 1 sleep for 1 seconds: 3
Thread 2 sleep for 1 seconds: 4
Thread 0 sleep for 1 seconds: 2
Received: 2
Received: 3
Received: 1
Thread 1 sleep for 1 seconds: 4
Thread 2 sleep for 1 seconds: 5
Thread 0 sleep for 1 seconds: 3
Received: 3
Received: 4
Received: 2
Thread 4 sleep for 4 seconds: 5
Thread 3 sleep for 2 seconds: 5
Received: 4
Received: 4
Thread 1 sleep for 1 seconds: 5
Thread 2 sleep for 1 seconds: 6
Thread 0 sleep for 1 seconds: 4
Received: 4
Received: 5
Received: 3
Received: 5
Received: 6
Received: 4
Thread 3 sleep for 2 seconds: 6
Received: 5
Thread 4 sleep for 4 seconds: 6
Received: 5
Thread 3 sleep for 2 seconds: 7
Received: 6
Received: 7
Thread 4 sleep for 4 seconds: 7
Received: 6
Thread 4 sleep for 4 seconds: 8
Received: 7
Received: 8
Done
```

Alternatively, with vectors:

```rust
extern crate rand;

use std::sync::mpsc::sync_channel;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use rand::distributions::{IndependentSample, Range};

fn main() {
  let max_count = 25;
  let max_buffer = 8;

  let (tx, rx) = sync_channel(max_buffer);
  let nums: Vec<usize> = (0..max_count).collect();
  let data = Arc::new(Mutex::new(nums));
  let counter = Arc::new(Mutex::new(0));

  let handle = thread::spawn(move || {
    let mut count = 0;
    'outer: loop {
      match rx.recv() {
        Ok(val) => {
          // println!("Received: {:?}", val);
          count += 1;
        },
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

  let between = Range::new(1, 5); // Maximum sleep is 4s - that is also the time required for the program to complete
  let mut rng = rand::thread_rng();

  while data.lock().unwrap().len() != 0 {
  // for i in 0..5 {
    let duration = between.ind_sample(&mut rng);
    let (tx, data, counter) = (tx.clone(), data.clone(), counter.clone());
  
    let handle = thread::spawn(move || {
      // println!("Spawned thread {}", i);
      let mut inner_handles = vec![];

      'inner: for _j in 0..8 {
        let (tx, data, counter) = (tx.clone(), data.clone(), counter.clone());

        // This is a blocking operation, since it is mutating the vector
        let mut new_v = data.lock().unwrap();
        match new_v.pop() {
          Some(val) => {
            // Place do_work, a computationally long process in another thread to allow
            // the 'inner loop to move faster
            let mut counter_val = counter.lock().unwrap();
            *counter_val += 1;
            let is_exceeded = *counter_val >= 8;
            let inner_handle = thread::spawn(move|| {
              match tx.send(do_work(val, duration, is_exceeded)) {
                Ok(_) => (),//println!("Send {}", val)
                Err(err) => println!("SendError: {:?}", err),
              }
            });
            inner_handles.push(inner_handle);
          }
          None => break 'inner
        }
      };

      for inner_handle in inner_handles {
        inner_handle.join().unwrap();
      }
    });

    handles.push(handle);
  }

  // For the sake of synchronization
  for handle in handles {
    handle.join().unwrap();
  }
}


fn do_work(i: usize, duration: u64, is_pending: bool) -> usize {
  if is_pending {
    println!("Tourists {} is waiting", i);
  } else {
    println!("Tourists {} is online", i);
  }
  
  thread::sleep(time::Duration::from_secs(duration));
  println!("Tourists {} is done after {} hours", i, duration);
  i
}
```

Output:

```
Tourists 24 is online
Tourists 23 is online
Tourists 22 is online
Tourists 21 is online
Tourists 20 is online
Tourists 19 is online
Tourists 18 is online
Tourists 17 is waiting
Tourists 16 is waiting
Tourists 15 is waiting
Tourists 14 is waiting
Tourists 13 is waiting
Tourists 12 is waiting
Tourists 11 is waiting
Tourists 10 is waiting
Tourists 9 is waiting
Tourists 8 is waiting
Tourists 7 is waiting
Tourists 6 is waiting
Tourists 5 is waiting
Tourists 4 is waiting
Tourists 3 is waiting
Tourists 2 is waiting
Tourists 1 is waiting
Tourists 0 is waiting
Tourists 24 is done after 1 hours
Tourists 20 is done after 1 hours
Tourists 18 is done after 1 hours
Tourists 15 is done after 1 hours
Tourists 13 is done after 1 hours
Tourists 12 is done after 1 hours
Tourists 9 is done after 1 hours
Tourists 7 is done after 1 hours
Tourists 5 is done after 1 hours
Tourists 2 is done after 1 hours
Tourists 0 is done after 1 hours
Tourists 23 is done after 2 hours
Tourists 19 is done after 2 hours
Tourists 14 is done after 2 hours
Tourists 8 is done after 2 hours
Tourists 1 is done after 2 hours
Tourists 22 is done after 3 hours
Tourists 17 is done after 3 hours
Tourists 4 is done after 3 hours
Tourists 11 is done after 3 hours
Tourists 6 is done after 3 hours
Tourists 21 is done after 4 hours
Tourists 16 is done after 4 hours
Tourists 3 is done after 4 hours
Tourists 10 is done after 4 hours
Done
```

If you notice, the jobs with the shortest time (1hr) will complete first.