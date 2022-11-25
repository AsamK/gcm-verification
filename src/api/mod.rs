use crate::errors::Error;
use crate::lib::mailauth::{generate_firebase_token, request_email_verification};
use crate::lib::*;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router, Server};
use http::header::CONTENT_TYPE;
use http::{Method, StatusCode};
use hyper::client::connect::HttpConnector;
use hyper::Client;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Debug)]
struct EmailRequest {
    email: String,
}

async fn get_request_email_verification(
    State(client): State<Client<HttpsConnector<HttpConnector>>>,
    Json(EmailRequest { email }): Json<EmailRequest>,
) -> Result<impl IntoResponse, Error> {
    request_email_verification(&client, &email).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct TokenRequest {
    email: String,
    link: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub user_id: String,
}

async fn get_firebase_token(
    State(client): State<Client<HttpsConnector<HttpConnector>>>,
    Json(TokenRequest { email, link }): Json<TokenRequest>,
) -> Result<impl IntoResponse, Error> {
    let res = generate_firebase_token(&client, &email, &link).await?;
    Ok(Json(TokenResponse {
        access_token: res.access_token,
        user_id: res.user_id,
    }))
}

pub async fn run() -> Result<(), anyhow::Error> {
    let addr: std::net::SocketAddr = "127.0.0.1:9090".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let connector = HttpsConnectorBuilder::new()
        .with_tls_config(tls::get_client_config())
        .https_only()
        .enable_http1()
        .build();
    let client: Client<_, hyper::Body> = Client::builder().build(connector);

    let app = Router::new()
        .route("/account", get(create_account))
        .route("/verification", post(get_verification))
        .route("/email/request", post(get_request_email_verification))
        .route("/email/confirm", post(get_firebase_token))
        .with_state(client)
        .layer(cors);

    Ok(Server::bind(&addr).serve(app.into_make_service()).await?)
}
