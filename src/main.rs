use std::cmp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::io::Write;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        false
    } else {
        let max = (n as f64).sqrt() as u64 + 1;
        for i in 2..max {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

fn parallel_is_prime_range(
    n: u64,
    start: u64,
    end: u64,
    flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> bool {
    for i in start..end {
        if n % i == 0 {
            flag.store(false, Ordering::Relaxed);
            return false;
        } else {
            if ((i & 0xfff) == 0) && (!flag.load(Ordering::Relaxed)) {
                return false;
            }
        }
    }
    true
}

fn parallel_is_prime(n: u64, num_threads: u64) -> bool {
    if n < 2 {
        false
    } else if n < num_threads * 1000000000 {
        return is_prime(n);
    } else {
        let mut start = 2;
        let mut end = start;
        let max = (n as f64).sqrt() as u64 + 1;
        let step: u64 = max / num_threads;

        let flag = Arc::new(AtomicBool::new(true));

        let mut threads = vec![];

        while start < max {
            start = end;
            end = cmp::min(end + step, max);

            let this_flag = Arc::clone(&flag);
            threads.push(thread::spawn(move || {
                parallel_is_prime_range(n, start, end, this_flag);
            }));
        }

        for thrd in threads {
            thrd.join().unwrap();
        }
        return flag.load(Ordering::Relaxed);
    }
}

fn main() {
    let start = 2;
    let end = u64::MAX;
    println!("finding primes from {} to {}", start, end);

    for i in start..=end {
        if parallel_is_prime(i, 4) {
            println!("{}", i);
            std::io::stdout().flush().unwrap();
        }
    }
}
