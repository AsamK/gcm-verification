use http::header::HeaderName;
use http::Uri;
use hyper::client::HttpConnector;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::json;

use crate::errors::Error;

const BASE_URL: &str = "https://www.googleapis.com";
const KEY: &str = "AIzaSyDFUC30aJbUREs-vKefE6QmvoVL0qqOv60";

const ANDROID_PACKAGE: &str = "com.tellm.android.app";
const ANDROID_CERT: &str = "A4A8D4D7B09736A0F65596A868CC6FD620920FB0";
const CLIENT_VERSION: &str = "Android/Fallback/X21000001/FirebaseCore-Android";

pub async fn request_email_verification(
    client: &Client<HttpsConnector<HttpConnector>>,
    email: &str,
) -> Result<(), Error> {
    let url =
        format!("{BASE_URL}/identitytoolkit/v3/relyingparty/getOobConfirmationCode?key={KEY}");

    let payload = json! ({
        "requestType": 6,
        "email": email,
        "androidInstallApp": true,
        "canHandleCodeInApp": true,
        "continueUrl": "https://jodel.com/app/magic-link-fallback",
        "androidPackageName": "com.tellm.android.app",
        "androidMinimumVersion": "5.116.0",
    });

    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(
            HeaderName::from_static("x-android-package"),
            HeaderValue::from_static(ANDROID_PACKAGE),
        )
        .header(
            HeaderName::from_static("x-android-cert"),
            HeaderValue::from_static(ANDROID_CERT),
        )
        .header(
            HeaderName::from_static("x-client-version"),
            HeaderValue::from_static(CLIENT_VERSION),
        )
        .body(payload.to_string().into())?;

    let res = client.request(req).await?;
    if res.status() != 200 {
        return Err(Error::Msg("Failed to request verification"));
    }

    Ok(())
}

#[derive(Deserialize)]
struct LinkFromEmailQuery {
    link: String,
}

#[derive(Deserialize)]
struct LinkFromEmailLinkQuery {
    #[serde(rename = "oobCode")]
    oob_code: String,
}

pub async fn generate_firebase_token(
    client: &Client<HttpsConnector<HttpConnector>>,
    email: &str,
    link_from_email: &str,
) -> Result<FirebaseTokenResponse, Error> {
    let url = link_from_email.parse::<Uri>()?;
    let url_link_part =
        serde_urlencoded::from_str::<LinkFromEmailQuery>(url.query().unwrap_or(""))?.link;

    let url_link = url_link_part.parse::<Uri>()?;
    let oob_token =
        serde_urlencoded::from_str::<LinkFromEmailLinkQuery>(url_link.query().unwrap_or(""))?
            .oob_code;

    let firebase_token = redeem_oob(client, email, &oob_token).await?.refresh_token;
    let fresh_token = refresh_tokens(client, &firebase_token).await?;

    Ok(fresh_token)
}

#[derive(Deserialize)]
struct EmailLinkSigninResponse {
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}

async fn redeem_oob(
    client: &Client<HttpsConnector<HttpConnector>>,
    email: &str,
    oob_code: &str,
) -> Result<EmailLinkSigninResponse, Error> {
    let url = format!("{BASE_URL}/identitytoolkit/v3/relyingparty/emailLinkSignin?key={KEY}");

    let payload = json! ({
        "email": email,
        "oobCode": oob_code,
    });

    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(
            HeaderName::from_static("x-android-package"),
            HeaderValue::from_static(ANDROID_PACKAGE),
        )
        .header(
            HeaderName::from_static("x-android-cert"),
            HeaderValue::from_static(ANDROID_CERT),
        )
        .header(
            HeaderName::from_static("x-client-version"),
            HeaderValue::from_static(CLIENT_VERSION),
        )
        .body(payload.to_string().into())?;

    let res = client.request(req).await?;
    if res.status() != 200 {
        return Err(Error::Msg("Failed to redeem oob"));
    }
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body: EmailLinkSigninResponse = serde_json::from_slice(&body_bytes)?;

    Ok(body)
}

#[derive(Deserialize)]
pub struct FirebaseTokenResponse {
    pub access_token: String,
    pub user_id: String,
}

async fn refresh_tokens(
    client: &Client<HttpsConnector<HttpConnector>>,
    refresh_token: &str,
) -> Result<FirebaseTokenResponse, Error> {
    let url = format!("https://securetoken.googleapis.com/v1/token?key={KEY}");

    let payload = json! ({
        "grantType": "refresh_token",
        "refreshToken": refresh_token,
    });

    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(
            HeaderName::from_static("x-android-package"),
            HeaderValue::from_static(ANDROID_PACKAGE),
        )
        .header(
            HeaderName::from_static("x-android-cert"),
            HeaderValue::from_static(ANDROID_CERT),
        )
        .header(
            HeaderName::from_static("x-client-version"),
            HeaderValue::from_static(CLIENT_VERSION),
        )
        .body(payload.to_string().into())?;

    let res = client.request(req).await?;
    if res.status() != 200 {
        return Err(Error::Msg("Failed to refresh tokens"));
    }
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body: FirebaseTokenResponse = serde_json::from_slice(&body_bytes)?;

    Ok(body)
}
