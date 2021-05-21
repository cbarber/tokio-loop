use std::time::Duration;
use tokio::time::Instant;
use tokio::{spawn, task::JoinHandle};

#[tokio::main]
async fn main() {
    // no await in here
    let handle_1: JoinHandle<()> = spawn(async {
        let start = Instant::now();
        let mut tick = Instant::now();
        let mut i = 0u64;
        while start.elapsed() < Duration::from_millis(400) {
            i = i.wrapping_add(1);
            if tick.elapsed() > Duration::from_millis(40) {
                println!("40ms-tick: {:?}: {}", start.elapsed(), i);
                tick = Instant::now();
            }
        }
        println!("1000s-tick: {:?}: {}", start.elapsed(), i);
    });

    // a tokio executor sleep in here. not the same as OS sleep
    let handle_2: JoinHandle<()> = spawn(async {
        let mut start = Instant::now();
        for i in 1u8..10 {
            tokio::time::delay_for(Duration::from_millis(10)).await;
            println!("10ms-tick: {:?}: {}", start.elapsed(), i);
            start = Instant::now();
        }
    });

    tokio::try_join!(handle_1, handle_2).unwrap();
}
