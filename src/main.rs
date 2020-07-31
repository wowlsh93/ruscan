
mod analyzer;
mod crawler;

use std::{thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};
use std::sync::Arc;
use std::sync::atomic::Ordering;


fn main() {

    let ch : (Sender<String>, Receiver<String>) = unbounded();

    let  anal  = analyzer::Analyzer::new();
    anal.start(&ch.1);

    let  craw = crawler::Crawler::new();
    craw.start(&ch.0);


    // set the control-c handler
    ctrlc::set_handler(move || {
        info!("Got Ctrl-C handler signal. Stopping...");
        shared_ge_controlc.stop_signal.store(true, Ordering::SeqCst);
        if !shared_ge_controlc.cfg.scan {
            std::process::exit(0);
        }
    }).expect("Error setting Ctrl-C handler");

    let mut count = 0u32;
    loop {
        count += 1;
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        println!("{}", count);

    }

}
