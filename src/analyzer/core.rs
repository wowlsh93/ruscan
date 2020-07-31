


use std::{thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};
use std::sync::Arc;


pub struct Analyzer {


}


impl Analyzer {

    pub fn new(receiver: Receiver<String>) -> Analyzer {

        let anal = Analyzer { };

        thread::spawn(move || {

            loop {
                select! {
                        recv(receiver) -> msg => {
                                 if msg == Err(RecvError) {
                                   println!("err_r1");
                                 }else{
                                   println!("{}", msg.unwrap());
                                 }}
                                ,
                        default(Duration::from_secs(3)) => println!("timed out")
                    }
            }

        });

        return anal;
    }


}