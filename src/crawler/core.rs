

use std::{thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};
use std::sync::Arc;

#[macro_use] extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use std::io::Read;
use hyper::{Client};
use hyper::header::{Headers, ContentType};
use rustc_serialize::{Encodable, json};
use rustc_serialize::json::Object;


#[derive(RustcDecodable, RustcEncodable)]
struct Param {
    num: String,
    detail: bool,
}
#[derive(RustcDecodable, RustcEncodable)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: params: Vec<Json>,
    id: String,
}

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

    fn post_json<T>(url: &str, payload: &T) -> hyper::Result<String>
        where T: Encodable {
        let client = Client::new();
        let body = json::encode(payload).unwrap();
        let mut response = try!(client.post(url).header(ContentType::json()).body(&body[..]).send());
        let mut buf = String::new();
        try!(response.read_to_string(&mut buf));
        Ok(buf)
    }

    fn getBlock(){

        let request = RpcRequest {
            jsonrpc: "2.0".to_owned(),
            method: "eth_getBlockByNumber".to_owned(),
            params: params: vec![Json::String("1".into()), Json::Boolean(true)],
            id: "1".to_owned(),
        };
        println!("{}", post_json("https://mainnet.infura.io/v3/9366572d83da40e4b827a664e6194e06", &request).unwrap());

    }
}
