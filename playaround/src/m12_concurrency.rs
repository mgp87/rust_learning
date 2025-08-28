#[cfg(test)]
mod test {
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread::{JoinHandle, spawn};

    #[test]
    fn test_concurrency() {
        let file_mutex: Arc<Mutex<File>> = Arc::new(Mutex::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("increments.txt")
                .unwrap(),
        ));
        // Mutex allows thread-safe access to the file from multiple threads
        // Arc means atomic reference counting, and being thread safe, it allows multiple threads to own the Mutex

        let mut handlers: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let file_mutex_clone: Arc<Mutex<File>> = Arc::clone(&file_mutex);
            let handle: JoinHandle<()> = spawn(move || {
                let mut file: MutexGuard<File> = file_mutex_clone.lock().unwrap(); // lock the mutex to get access to the file
                writeln!(file, "{}", i).unwrap(); // write the increment to the file
            });
            handlers.push(handle);
        }

        for handler in handlers {
            handler.join().unwrap(); // wait for all threads to finish
        }
    }
}
