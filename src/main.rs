use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc}, thread::JoinHandle, thread};
use std::sync::Mutex;

static INDEX: AtomicUsize = AtomicUsize::new(0);

fn impl_by_atomicusize() {
    let words = ["hello", " ", "world", "!"];
    let len = words.len();
    let point = Arc::new(words);
    let mut thread_vec: Vec<JoinHandle<()>> = Vec::with_capacity(len);

    for _ in 0..len {
        let point_clone = point.clone();
        let thread = thread::spawn(move || {
            print!("{}", point_clone[INDEX.load(Ordering::Relaxed)]);
            INDEX.fetch_add(1, Ordering::Relaxed);
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread.join().unwrap();
    }
    println!("{}", INDEX.load(Ordering::Relaxed));
}

fn impl_by_mutex() {
    let words = ["hello", " ", "world", "!"];
    let len = words.len();
    let point = Arc::new(words);
    let index_lock = Arc::new(Mutex::new(0 as usize));
    let mut thread_vec: Vec<JoinHandle<()>> = Vec::with_capacity(len);

    for _ in 0..len {
        let point_clone = point.clone();
        let index_lock_clone = index_lock.clone();
        let thread = thread::spawn(move || {
            let mut i = index_lock_clone.lock().unwrap();
            print!("{}", point_clone[*i]);
            *i += 1;
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread.join().unwrap();
    }
}

fn main() {
    // impl_by_atomicusize();
    impl_by_mutex();

}