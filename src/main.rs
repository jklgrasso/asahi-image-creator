use clap::{self, Error};
use std::path::{Path, PathBuf};

use crate::create::Image;

mod create;
mod macos;

fn main() -> Result<(), Error> {
    let loopback = Image::loopback();
    let exists = Path::exists(&PathBuf::from("/dev/loop0"));

    println!("ISO provided: {}", loopback.name.trim_start_matches("../"));

    if exists == true {
        println!("Loopback device already found. Detaching");
        loopback.detach;
        println!("Starting loopback");
        loopback.loopdevice;
    } else {
        loopback.loopdevice;
        println!("Starting loopback");
        loopback.detach;
        println!("Detaching loopback device")
    }

    loop {
        if ctrlc::Signal::SIGIO == ctrlc::Signal::SIGKILL {
            println!("caught");
            println!("Stopping loopback");
            loopback.detach;
            break;
        }
    }

    Ok(())
}
