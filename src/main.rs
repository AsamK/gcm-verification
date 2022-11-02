use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;

use self::lib::*;

mod api;
mod errors;
mod lib;
mod protos;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}

async fn run() -> Result<(), anyhow::Error> {
    self::api::run().await?;
    println!("Starting Jodel GCM verification server");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        let connector = HttpsConnectorBuilder::new()
            .with_tls_config(tls::get_client_config())
            .https_only()
            .enable_http1()
            .build();
        let client: Client<_, hyper::Body> = Client::builder().build(connector);

        let response = request(&client).await.unwrap();
        let response = serde_json::to_string(&response).unwrap();
        println!("{}", response);
    } else if args.len() == 3 {
        let account = AndroidAccount {
            android_id: args.get(1).unwrap().parse().unwrap(),
            security_token: args.get(2).unwrap().parse().unwrap(),
        };
        let code = read(&account).await.unwrap();
        let response = serde_json::to_string(&code).unwrap();
        println!("{}", response);
    } else {
        println!("Wrong command line args");
        std::process::exit(1);
    };
    Ok(())
}
