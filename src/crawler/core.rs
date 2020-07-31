

use std::{thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};
use std::sync::Arc;

pub struct Crawler {

    pub sender_ : Sender<String>,
}

impl Crawler {

    pub fn new(sender: Sender<String>) -> Crawler {

        let c = Crawler { sender_ : sender };


        thread::spawn(move || {

            loop {
                getBlock();
                c._sender(String::from("block")).unwrap();

                let ten_millis = time::Duration::from_millis(1000);
                thread::sleep(ten_millis);

            }

        });


        return c;
    }

    fn getBlock(){

    }
}
