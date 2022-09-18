use crate::errors::Error;
use crate::lib::*;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router, Server};
use http::header::CONTENT_TYPE;
use http::{Method, StatusCode};
use hyper::client::connect::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Serialize;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("Error {:?}", self);
        let body = Json(json!({
            "error": self.to_string(),
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Serialize)]
struct VerificationApiResponse {
    verification: VerificationResponse,
}

async fn create_account(
    State(client): State<Client<HttpsConnector<HttpConnector>>>,
) -> Result<impl IntoResponse, Error> {
    let response = request(&client).await?;
    Ok(Json(response))
}

async fn get_verification(
    Json(body): Json<AndroidAccountSerDe>,
) -> Result<impl IntoResponse, Error> {
    let account = AndroidAccount {
        android_id: body
            .android_id
            .parse()
            .map_err(|_e| Error::Msg("Invalid android_id number"))?,
        security_token: body
            .security_token
            .parse()
            .map_err(|_e| Error::Msg("Invalid security_token number"))?,
    };
    let code = read(&account).await?;
    Ok(Json(VerificationApiResponse { verification: code }))
}

pub async fn run() -> Result<(), anyhow::Error> {
    let addr: std::net::SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let client = Client::builder().build(HttpsConnector::new());

    let app = Router::with_state(client)
        .route("/account", get(create_account))
        .route("/verification", post(get_verification))
        .layer(cors);

    Ok(Server::bind(&addr).serve(app.into_make_service()).await?)
}
