#![feature(await_macro, async_await, futures_api)]
#![feature(try_trait)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate serde_derive;

use hyper::Client;
use hyper_tls::HttpsConnector;

use self::lib::*;

mod lib;
mod protos;
mod errors;

fn main() {
    println!("Starting Jodel GCM verification server");
    let client = Client::builder()
        .keep_alive(false)
        .build(HttpsConnector::new(4).unwrap());

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        tokio::run_async(async move {
            await!(request(&client));
        });
    } else if args.len() == 3 {
        let account = AndroidAccount {
            android_id: args.get(1).unwrap().parse().unwrap(),
            security_token: args.get(2).unwrap().parse().unwrap(),
        };
        tokio::run_async(async move {
            await!(read(&account));
        });
    } else {
        println!("Wrong command line args");
        std::process::exit(1);
    };
}