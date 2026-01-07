use std::sync::mpsc; //channels for passing messages amongs threads

use std::sync::Arc; //This is the thread safe version of smartpointer RC
use std::thread;
use std::time::Duration;
use thread::JoinHandle;

use std::sync::Mutex;
//use std::sync::MutexGuard;

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); //This would tell the main thread to wait for the spawned thread.

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_sample();

    pass_data_amongs_thread();

    pass_data_shared_state();

    mutex_sharedata_multiple_thread();
}

fn thread_sample() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    //The move keyword is used to move the overnership of the list to the thread.
    let handle: JoinHandle<()> = thread::spawn(move || {
        println!(
            "This is the vector printed in here from the thread {:?}",
            list
        );
    });

    handle.join().unwrap();
}

//You can have multiple producer but only one reciever

fn pass_data_amongs_thread() {
    let v: Vec<String> = vec![
        String::from("My"),
        String::from("is"),
        String::from("Jide"),
        String::from("Akindejoye"),
    ];

    let mut v2: Vec<String> = Vec::new();
    v2.push("Gods".to_string());
    v2.push("are".to_string());
    v2.push("to".to_string());
    v2.push("be".to_string());
    v2.push(String::from("blamed"));

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); //creating another transmitter

    thread::spawn(move || {
        // let msg: String = String::from("hi");

        for value in v {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // println!("Let try this illegal print operation {}", msg); This is illegal cos the msg has been sent to reciever
    });

    //This is the second transmitter
    thread::spawn(move || {
        for value in v2 {
            tx2.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //A single reciever getting messages sent from bot transmitters
    for recieved in rx {
        println!("{}", recieved);
    }
}

fn pass_data_shared_state() {
    //Mutex are used to power this.
    //You must follow this rules. 1. YOu need to acquire a lock to get access to data.
    //2. You need to release that lock so other threads can get access to the data
    let m: Mutex<i32> = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_sharedata_multiple_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
