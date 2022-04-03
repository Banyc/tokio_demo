use tokio;

fn thread_tx1(tx1: tokio::sync::mpsc::Sender<i32>) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("thread_tx1: begin");
        for i in 0..10 {
            println!("thread_tx1: loop begin");
            // sleep for 1 second
            // std::thread::sleep(std::time::Duration::from_millis(1000));
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // send a message
            tx1.send(i).await.unwrap();
        }
        println!("thread_tx1: end");
    })
}

fn thread_tx2(tx2: tokio::sync::mpsc::Sender<bool>) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("task_tx2: begin");
        for i in 0..10 {
            println!("task_tx2: loop begin");
            // sleep for 1 second
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // send a message
            tx2.send(i % 2 == 0).await.unwrap();
        }
        println!("task_tx2: end");
    })
}

fn thread_rx(
    mut rx1: tokio::sync::mpsc::Receiver<i32>,
    mut rx2: tokio::sync::mpsc::Receiver<bool>,
) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        loop {
            tokio::select! {
                val = rx1.recv() => {
                    if let Some(val) = val {
                        println!("rx: channel1: {}", val);
                    } else {
                        println!("rx: channel1: closed");
                        break;  // exit loop
                    }
                }
                val = rx2.recv() => {
                    if let Some(val) = val {
                        println!("rx: channel2: {}", val);
                    } else {
                        println!("rx: channel2: closed");
                        break;  // exit loop
                    }
                }
            }
        }
    })
}

fn main() {
    let (tx1, rx1) = tokio::sync::mpsc::channel::<i32>(1);
    let (tx2, rx2) = tokio::sync::mpsc::channel::<bool>(1);

    std::thread::spawn(move || thread_tx1(tx1));
    std::thread::spawn(move || thread_tx2(tx2));

    std::thread::spawn(move || thread_rx(rx1, rx2))
        .join()
        .unwrap();
}
