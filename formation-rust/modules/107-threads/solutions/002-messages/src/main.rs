use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    step4();
}

fn step1() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let message = receiver.recv().unwrap();
        println!("Received: {}", &message);
    });
    sender.send(String::from("Hello!")).unwrap();
    handle.join().unwrap();
}

fn step2() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for message in receiver.iter() {
            println!("Received: {}", &message);
        }
    });
    sender.send(String::from("Hello 1!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 2!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 3!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 4!")).unwrap();
    handle.join().unwrap();
}

fn step3_a() {
    let (sender, receiver) = mpsc::channel();
    let _ = thread::spawn(move || {
        for message in receiver.iter() {
            println!("Received: {}", &message);
        }
    });
    sender.send(String::from("Hello 1!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 2!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 3!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 4!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
}

fn step3_b() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for message in receiver.iter() {
            if &message == "STOP" {
                break;
            }
            println!("Received: {}", &message);
        }
    });
    sender.send(String::from("Hello 1!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 2!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 3!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 4!")).unwrap();
    sender.send(String::from("STOP")).unwrap();
    handle.join().unwrap();
}

fn step3_c() {
    enum MyMessage {
        Stop,
        Msg(String),
    }
    let (sender, receiver) = mpsc::channel();
    let listener = thread::spawn(move || {
        for message in receiver.iter() {
            match message {
                MyMessage::Stop => {
                    break;
                }
                MyMessage::Msg(inner) => {
                    println!("Received: {}", &inner);
                }
            }
        }
    });
    sender
        .send(MyMessage::Msg(String::from("Hello 1 from main!")))
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender
        .send(MyMessage::Msg(String::from("Hello 2 from main!")))
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender
        .send(MyMessage::Msg(String::from("Hello 3 from main!")))
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender
        .send(MyMessage::Msg(String::from("Hello 4 from main!")))
        .unwrap();
    sender.send(MyMessage::Stop).unwrap();
    listener.join().unwrap();
}

fn step3_d() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || loop {
        match receiver.recv_timeout(Duration::from_millis(1500)) {
            Ok(message) => {
                println!("Received: {}", &message);
            }
            Err(_) => {
                break;
            }
        };
    });
    sender.send(String::from("Hello 1!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 2!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 3!")).unwrap();
    thread::sleep(Duration::from_millis(1000));
    sender.send(String::from("Hello 4!")).unwrap();
    handle.join().unwrap();
}

fn step4() {
    let (sender, receiver) = mpsc::channel();
    let listener = thread::spawn(move || {
        for message in receiver.iter() {
            if &message == "STOP" {
                break;
            }
            println!("Received: {}", &message);
        }
    });
    let sender2 = sender.clone();
    let producer = thread::spawn(move || {
        sender2
            .send(String::from("Hello 1 from producer!"))
            .unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender2
            .send(String::from("Hello 2 from producer!"))
            .unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender2
            .send(String::from("Hello 3 from producer!"))
            .unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender2
            .send(String::from("Hello 4 from producer!"))
            .unwrap();
    });
    {
        sender.send(String::from("Hello 1 from main!")).unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender.send(String::from("Hello 2 from main!")).unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender.send(String::from("Hello 3 from main!")).unwrap();
        thread::sleep(Duration::from_millis(1000));
        sender.send(String::from("Hello 4 from main!")).unwrap();
    }
    producer.join().unwrap();
    sender.send(String::from("STOP")).unwrap();
    listener.join().unwrap();
}
