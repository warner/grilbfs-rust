extern crate fuse;
//extern crate env_logger;
extern crate libc;
extern crate time;
mod grilbfs;

use std::env;

fn main() {
    //env_logger::init().unwrap();
    let mountpoint = env::args_os().nth(1).unwrap();
    fuse::mount(grilbfs::GrilbFS, &mountpoint, &[]).unwrap();
}


