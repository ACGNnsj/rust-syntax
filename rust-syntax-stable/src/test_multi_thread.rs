use public_function::prime::{
    is_prime,
};

fn count_prime_by_thread(n: u32, total_thread: u32, thread_num: u32) -> u32 {
    let mut count = 0;
    let start = 1 + total_thread * thread_num * 2;
    let interval = total_thread * (total_thread - 1) * 2;
    let mut i = start;
    while i <= n {
        for _j in 0..total_thread {
            if i > n {
                return count;
            } else if is_prime(i) {
                count += 1;
            }
            i += 2;
        }
        i += interval;
    }
    count
}

use std::{
    thread,
    sync::{
        Arc, mpsc, Mutex,
        atomic::{AtomicU32, AtomicUsize, Ordering},
    },
};

use public_struct::error::MyError;

#[test]
fn test() {
    // let mut result = Vec::<u32>::new();
    static TOTAL_THREAD: u32 = 10;
    // let mut sum = 1;
    // let i = 1;
    static N: u32 = 100000000;
    // let mut long_live_i: Vec<u32> = vec![];
    let sum = Arc::new(AtomicU32::new(1));
    let mut handles = vec![];
    // let num = Box::new(0);
    let counter = Arc::new(AtomicU32::new(0));
    let start = std::time::Instant::now();
    let (tx, rx) = mpsc::channel();
    for i in 0..TOTAL_THREAD {
        let arc_i = Arc::new(Mutex::new(i));
        let arc_sum = Arc::clone(&sum);
        let arc_counter = counter.clone();
        let arc_start = Arc::new(Mutex::new(start));
        // long_live_i.push(i);
        // let static_i: &'static u32 = &mut long_live_i[i as usize];
        let tx_thread = tx.clone();
        let handle = thread::spawn(move || {
            let mut thread_i = *arc_i.lock().unwrap();
            let count = count_prime_by_thread(N, TOTAL_THREAD, thread_i);
            thread_i += 1;
            let mut thread_sum = arc_sum.fetch_add(count, Ordering::SeqCst);
            thread_sum += count;
            println!("thread_i:{},count:{}", thread_i, count);
            let mut previous_thread_counter = arc_counter.fetch_add(1, Ordering::SeqCst);
            // thread_counter += 1;
            println!("thread_counter:{}", previous_thread_counter + 1);
            if previous_thread_counter + 1 >= TOTAL_THREAD {
                let start = *arc_start.lock().unwrap();
                let end = std::time::Instant::now();
                println!("sum:{}", thread_sum);
                println!("time:{:?}", end - start);
                tx_thread.send(thread_sum).expect("TODO: panic message");
            }
            println!("continued");
            // tx.send(count).expect("TODO: panic message");
            // arc_result.push(count);
            /*if arc_result.len() == TOTAL_THREAD as usize {
                let mut sum = if N >= 2 {
                    1
                } else { 0 };
                for i in arc_result {
                    sum += i;
                }
                println!("sum:{}", sum);
            }*/
        });
        handles.push(handle);
        // let received = rx.recv().expect("TODO: panic message");
        // println!("received: {},current_time:{:?}", received, std::time::Instant::now());
    }
    for handle in handles {
        handle.join().expect("TODO: panic message");
    }
    let received = rx.recv().expect("TODO: panic message");
    println!("received: {},total_duration:{:?}", received, std::time::Instant::now() - start);
}