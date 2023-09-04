use embassy_executor::Spawner;
use embassy_rp::{
    uart,
    gpio::{Level, Output}
};
use embassy_time::{Duration, Timer};

/// MASCP Embassy Task
/// 
/// 