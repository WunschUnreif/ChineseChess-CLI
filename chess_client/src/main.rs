
#[macro_use] extern crate lazy_static;

mod config;
mod display;
mod server_comm;
mod data_model;

use std::{thread::sleep, time::Duration};

use display::View;

fn main() -> Result<(), std::io::Error> {
    
    let mut v = View::new()?;
    v.run();

    sleep(Duration::from_millis(10000));

    v.stop();
    Ok(())
}
