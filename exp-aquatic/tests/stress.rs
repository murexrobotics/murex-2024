use std::thread;
use std::time::Duration;

fn main() {
    let mut threads = vec![];
    for _ in 0..4 {
        threads.push(thread::spawn(|| {
            let mut i: f64 = 1.28347918237417234;
            loop {
                i = i * i * i * i;
            }
        }));
    }
    thread::sleep(Duration::from_secs(10));
    for thread in threads {
        thread.join().unwrap();
    }
}
