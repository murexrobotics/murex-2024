#![allow(dead_code, unused_must_use)]
#![feature(async_fn_in_trait)]

// mod input;
// mod error;
mod actions;
mod executor;

use tokio;
// use actions::Executor;
use executor::Exec;


#[tokio::main]
async fn main() {
    let mut exec = Exec::new().await;
    exec.run(); // Never ends

}
