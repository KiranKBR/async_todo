extern crate async_todo;
use async_todo::*;
use std::io::{self, BufRead};

#[tokio::main]
pub async fn main() {
    let mut tl: TodoList = TodoList::new();
    // let file = File::open("Flamespike-The-Crawler.in")?;
    // let reader = BufReader::new(file);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            runner::run_line(&l, &mut tl).await;
        }
    }

    // for line in reader.lines() {
    //     if let Ok(l) = line {
    //         let _ = runner::run_line(&l, &mut tl).await;
    //     }
    // }
    // Ok(())
}
