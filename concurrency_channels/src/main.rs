use tokio;

async fn task_tx1(tx1: tokio::sync::mpsc::Sender<i32>) {
    println!("task_tx1: begin");
    for i in 0..10 {
        println!("task_tx1: loop begin");
        // sleep for 1 second
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // send a message
        tx1.send(i).await.unwrap();
    }
    println!("task_tx1: end");
}

async fn task_tx2(tx2: tokio::sync::mpsc::Sender<bool>) {
    println!("task_tx2: begin");
    for i in 0..10 {
        println!("task_tx2: loop begin");
        // sleep for 1 second
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // send a message
        tx2.send(i % 2 == 0).await.unwrap();
    }
    println!("task_tx2: end");
}

async fn task_rx(
    mut rx1: tokio::sync::mpsc::Receiver<i32>,
    mut rx2: tokio::sync::mpsc::Receiver<bool>,
) {
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
}

fn main() {
    println!("Hello, world!");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let (tx1, rx1) = tokio::sync::mpsc::channel::<i32>(1);
        let (tx2, rx2) = tokio::sync::mpsc::channel::<bool>(1);

        tokio::spawn(async move {
            task_tx1(tx1).await;
        });
        tokio::spawn(async move {
            task_tx2(tx2).await;
        });

        let rx_handle = tokio::spawn(async move {
            task_rx(rx1, rx2).await;
        });
        // wait for the receiver to finish
        rx_handle.await.unwrap();
    })
}
