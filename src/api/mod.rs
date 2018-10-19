use crate::lib::*;
use http::{header, Method};
use hyper::Body;
use hyper::Client;
use hyper::client::connect::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio::await;
use tokio::prelude::*;
use tower_web::middleware::cors::{AllowedOrigins, CorsBuilder};
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct GcmVerificationResource {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

#[derive(Response)]
struct VerificationApiResponse {
    verification: VerificationResponse,
}

impl_web! {
    impl GcmVerificationResource {
        #[get("/account")]
        #[content_type("application/json")]
        async fn create_account(&self) -> RequestResponse {
            let response = await!(request(&self.client)).unwrap();
            response
        }

        #[post("/verification")]
        #[content_type("application/json")]
        async fn get_verification(&self, body: AndroidAccountSerDe) -> VerificationApiResponse {
            let account = AndroidAccount {
                android_id: body.android_id.parse().unwrap(),
                security_token: body.security_token.parse().unwrap(),
            };
            let code = await!(read(&account)).unwrap();
            VerificationApiResponse {
                verification: code,
            }
        }
    }
}

pub fn run() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let cors = CorsBuilder::new()
        .allow_origins(AllowedOrigins::Any { allow_null: true })
        .allow_headers(vec![header::CONTENT_TYPE])
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_credentials(false)
        .prefer_wildcard(true)
        .build();

    let client = Client::builder()
        .keep_alive(false)
        .build(HttpsConnector::new(4).unwrap());

    ServiceBuilder::new()
        .resource(GcmVerificationResource { client })
        .middleware(cors)
        .run(&addr)
        .unwrap();
}
