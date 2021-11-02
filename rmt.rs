// Basic example from https://doc.rust-lang.org/book/ch16-01-threads.html
use std::thread;
use std::time::Duration;

fn main() {
    // We can use closures with this - Need to look into more
    let handle = thread::spawn(|| {
	for i in 1..10 {
	    println!("Spawned (Multi) thread number {}.", i);
	    // Requires that the thread waits 1ms before printing again.
	    thread::sleep(Duration::from_millis(2));
	}
    });
    // handle.join().unwrap();
    for i in 1..5 {
	println!("Thread {} from main thread.", i);
	// Requires that the thread waits 3ms before printing again.
	thread::sleep(Duration::from_millis(3));
    }
    // "Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates."
    handle.join().unwrap();
}
