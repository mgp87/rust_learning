use std::thread;
use std::time::Duration;

fn run() {
    let handle = std::thread::spawn(move || {
        println!("Hello from a thread!");
    });

    // thread::sleep(Duration::from_secs(1));

    handle.join().unwrap();
    println!("Hello from main!");

    let v = vec![1,2,3];

    //let handle = std::thread::spawn(move || { // move forces ownership of the value used
    //      println!("{:?}", v);
    // });

    let mut thread_handlers = Vec::new();

    for e in v {
        thread_handlers.push(thread::spawn(move || println!("Thread {}", e)));
    }

    println!("Main thread");
    for handle in thread_handlers {
        handle.join().unwrap();
    }

     let (transmitter, receiver) = mpsc::channel();
    let val = String::from("Transmitting!");
    std::thread::spawn(move || {
        transmitter.send(val).unwrap(); // send takes ownership of the value sent
    });

    let msg = receiver.recv().unwrap(); // recv takes ownership of the value received
    println!("{}", msg);
    // println!("{}", vale); --> this fails because of ownership

    // channel --> transmitter sends data, receiver receives data
    let (transmitter, receiver) = mpsc::channel();

    let tx = transmitter.clone(); // creation of another producer
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"), String::from("From"), String::from("Original")];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });

    std::thread::spawn(move || {
        let vec = vec![String::from("Clone"), String::from("is"), String::from("Transmitting")];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver { // multiple producers one single consumer
        println!("{}", rec);
    }

    // Sync channel states how many values can be enqueued blocking operation once the queue is full until the receiver starts receiving values
    let (transmitter, receiver) = mpsc::sync_channel(1000);

    let tx = transmitter.clone(); // creation of another producer
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"), String::from("From"), String::from("Original")];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });

    std::thread::spawn(move || {
        let vec = vec![String::from("Clone"), String::from("is"), String::from("Transmitting")];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver { // multiple producers one single consumer
        println!("{}", rec);
    }

    // Send can be moved accross threads
    // Sync can be shared accross threads

    let rc1 = Arc::new(String::from("Test"));
    let rc2 = rc1.clone();

    std::thread::spawn(move || {
        rc2;
    });

    // Mutex means mutual exclusion and only allows access to one thread at a time mainting exclusive access by locking to that data
    // Arc is handy to share data accross threads
    // Mutex is handy for mutating data accross threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // let mut num2 = counter.lock().unwrap(); // this is a dead lock, this program will never finish execution due to this lock called dead lock
            *num +=1;
        }); // lock is given up
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());

    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    let _ = std::thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap(); // we acquire the lock here
        panic!(); // mutex is now poisoned
    }).join();

    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }

    *guard += 1;
    prinln!("{:?}", guard);
}