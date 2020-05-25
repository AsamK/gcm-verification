use crate::lib::*;
use hyper::client::connect::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct VerificationApiResponse {
    verification: VerificationResponse,
}

async fn create_account(
    client: Client<HttpsConnector<HttpConnector>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = request(&client).await.unwrap();
    Ok(warp::reply::json(&response))
}

async fn get_verification(body: AndroidAccountSerDe) -> Result<impl warp::Reply, warp::Rejection> {
    let account = AndroidAccount {
        android_id: body.android_id.parse().unwrap(),
        security_token: body.security_token.parse().unwrap(),
    };
    let code = read(&account).await.unwrap();
    Ok(warp::reply::json(&VerificationApiResponse {
        verification: code,
    }))
}

pub async fn run() {
    let addr: std::net::SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_method("POST")
        .allow_method("GET")
        .build();

    let client = Client::builder().build(HttpsConnector::new());

    let account_route = warp::path!("account")
        .and(warp::get())
        .and(warp::any().map(move || client.clone()))
        .and_then(create_account);

    let verification_route = warp::path!("verification")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(get_verification);

    let server = warp::serve(account_route.or(verification_route).with(cors));

    server.run(addr).await;
}
