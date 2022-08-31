use fake::Fake;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file_size: usize = 1048576;
    let file_number = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..100 {
        let file_number = Arc::clone(&file_number);
        let handle = thread::spawn(move || {
            let mut num = file_number.lock().unwrap();
            let fake_string = file_size.fake::<String>();
            let file_name = format!("file{}.txt", num);
            let mut file = File::create(file_name).unwrap();
            file.write_all(fake_string.as_bytes()).unwrap();

            *num += 1
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }
}