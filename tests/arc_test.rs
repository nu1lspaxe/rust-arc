use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

use rust_arc::arc::Arc;

#[test]
fn test_basic_arc_usage() {
    let my_arc = Arc::new(42);
    assert_eq!(*my_arc, 42);
}

#[test]
fn test_arc_clone_increments_refcount() {
    let a = Arc::new(123);
    let b = a.clone();
    let c = b.clone();
    assert_eq!(*a, 123);
    assert_eq!(*b, 123);
    assert_eq!(*c, 123);
    // drop will happen three times without double free
}

#[test]
fn test_arc_multi_threaded() {
    let arc = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let arc_clone = arc.clone();
        handles.push(thread::spawn(move || {
            arc_clone.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(arc.load(Ordering::SeqCst), 10);
}

#[test]
fn test_arc_static_str() {
    let a: Arc<&'static str> = Arc::new("hello");
    assert_eq!(*a, "hello");
}

#[test]
fn test_arc_string() {
    let msg = String::from("hello world");
    let arc = Arc::new(msg.clone());
    assert_eq!(*arc, msg);
}

#[test]
fn test_drop_last_arc() {
    use std::cell::Cell;

    thread_local! {
        static DROPPED: Cell<bool> = Cell::new(false);
    }

    struct Dropper;

    impl Drop for Dropper {
        fn drop(&mut self) {
            DROPPED.with(|flag| flag.set(true));
        }
    }

    {
        let a = Arc::new(Dropper);
        let b = a.clone();
        let _c = b.clone();
        // All clones dropped at the end of scope
    }

    assert!(DROPPED.with(|flag| flag.get()));
}
