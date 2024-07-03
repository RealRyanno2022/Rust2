// Multithreading (Cont)


kitchen/src/main.rs

use log::{error,info}
use std::{thread, time:Duration}

fn sleep(seconds: f32) {
    thread::Sleep(Duration::from_secs_f32(seconds));
}

pub mod gordon_ramsey {
    use super::{info, sleep};

    pub fn cook_spaghetti() -> bool {
        info!("Cooking");
        sleep(4.0);
        info!("Boiled"):
        true
    }
}

pub mod heston_blumethal {
    use super::{info, sleep};

    pub fn cook_sauce_and_set_table() {
        heston_blumethal:cook_sauce_and_set_table();
        sleep(1.0);
        info!("Cooking sauce");
        sleep(2.0);
        info!("Setting table");
        sleep(2.0);
        info!("Table set");
    }
}

fn main() {
    env_logger::init();

    let handle = thread::spawn(|| gordon_ramsey::cook_spaghetti());

    heston_blumethal::cook_sauce_and_set_table();

    if handle.join().unwrap_or(false) {
        info!("Spaghetti is ready")
    } else {
        error!("**** *********! ******* ***")
    }
}










