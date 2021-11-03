use std::thread;
use std::time::Duration;
// 2
use std::sync::mpsc;

fn main() {
    // 1: Running code simultaneously
    // We can use closures with this
    // TODO: Look into having handle take some arguments.
    let handle = | |(thread::spawn(| | {
	for i in 1..10 {
	    println!("Spawned (Multi) thread number {}.", i);
	    // Requires that the thread waits 1ms before printing again.
	    thread::sleep(Duration::from_millis(2));
	}
    }));
    // handle.join().unwrap();
    for i in 1..5 {
	println!("Thread {} from main thread.", i);
	// Requires that the thread waits 3ms before printing again.
	thread::sleep(Duration::from_millis(3));
    }
    // "Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates."
    handle().join().unwrap();

    // 2: Message Passing
    // "Do not commuicate by sharing memory; instead, share memory by communicating"
    // Channel => Transmitter and Receiver (Closed if either half is dropped)
    /* mpsc::channel() -> Multple Producer, Single Consumer

    This returns a tuple in which the first part is the transmitter  and the second is the receiver
    Typically named (tx, rx)

     */
    let (tx, rx) = mpsc::channel();

    // Move ensures that the closure takes ownership of all variables that it uses.
    // This means that it moves any variables that exist outside of the closure into the closure.
    // http://gradebot.org/doc/ipur/closure.html
    thread::spawn(move | | {
	// Sending One Value
	// let val = String::from("This is a value");

	// Sending Multiple Values
	let vals = vec![
	    String::from("This"),
	    String::from("is"),
	    String::from("a"),
	    String::from("value"),
	];
	// send returns a Result<T, E> meaning that we need to unwrap it.
	// In this case we used expect in order to send a specific message if unwrapping Result<T, E> panics.

	// Sending One Value
	// tx.send(val).expect("Failed to Send Message");

	// Sending Multiple Values
	for value in vals {
	    tx.send(value).expect("Failed to send value");
	    // When program is running, threads will wait 2 seconds before sending the next message (commented code).
	    // thread::sleep(Duration::from_secs(2));
	    thread::sleep(Duration::from_millis(1));
	}
	// println!("Val is {}", val);
	// ^ will fail because we cannot reference the value after we've sent it down the channel.
    });

    // recv also returns a Results<T, E> so we need to unwrap it as well.
    // When the channel closes, recv will return an error to show that there are no more values that are coming.
    // NOTE: try_recv can be used to return either an Ok value or an Err.
    // It's userful when threads need to do other work while waiting for something like messages.

    // Receiving one value
    // let received = rx.recv().expect("Failed to Receive Message");
    // println!("Got: {}", received);

    // Receiving Multiple Values
    for received in rx {
	println!("Got: {}", received);
    }

    let (mptx, mprx) = mpsc::channel();
    // Dealing with Multiple Producers
    // Cloning the Transmitter
    let mptx1 = mptx.clone();

    thread::spawn(move | | {
	let vals = vec![
	    String::from("This (mptx)"),
	    String::from("is (mptx)"),
	    String::from("Another (mptx)"),
	    String::from("Value (mptx)"),
	    String::from("from (mptx)"),
	    String::from("tx (mptx)")
	];

	for val in vals {
	    mptx.send(val).expect("Failed to send value");
	    thread::sleep(Duration::from_secs(2));
	}
    });

    thread::spawn(move | | {
	let vals = vec![
	    String::from("This (mptx1)"),
	    String::from("is (mptx1)"),
	    String::from("Another (mptx1)"),
	    String::from("Value (mptx1)"),
	    String::from("from (mptx1)"),
	    String::from("tx1 (mptx1)"),
	];

	for val in vals {
	    mptx1.send(val).expect("Failed to send value");
	    thread::sleep(Duration::from_secs(2));
	}
    });

    for received in mprx {
	println!("(Multiple Producer) Got: {}", received);
    }
}

