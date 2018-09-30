#![feature(await_macro, async_await, futures_api)]
#![feature(try_trait)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate serde_derive;

use self::protos::checkin;
use self::protos::mcs;
use bytes::BufMut;
use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use hyper::Client;
use hyper::{Body, Request};
use hyper_tls::HttpsConnector;
use native_tls::TlsConnector;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};
use std::borrow::Cow;
use std::default::Default;
use std::fmt::Display;
use std::net::ToSocketAddrs;
use tokio::net::TcpStream;

mod protos;

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

#[derive(Debug)]
struct AndroidAccount {
    android_id: i64,
    security_token: u64,
}

#[derive(Debug)]
pub enum Error {
    NoId,
    NoToken,
    VarInt,
    Hyper(hyper::Error),
    Http(http::Error),
    HttpHeader(http::header::InvalidHeaderValue),
    None(std::option::NoneError),
    Serde(serde_urlencoded::ser::Error),
    Addr(std::net::AddrParseError),
    Protobuf(quick_protobuf::errors::Error),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        ""
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Error {
        Error::Addr(err)
    }
}

impl From<std::option::NoneError> for Error {
    fn from(err: std::option::NoneError) -> Error {
        Error::None(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Error {
        Error::Serde(err)
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(err: http::header::InvalidHeaderValue) -> Error {
        Error::HttpHeader(err)
    }
}

impl From<quick_protobuf::errors::Error> for Error {
    fn from(err: quick_protobuf::errors::Error) -> Error {
        Error::Protobuf(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

fn get_checkin_request_payload<'a>() -> checkin::CheckinRequest<'a> {
    let mut checkin_request = checkin::CheckinRequest::default();
    checkin_request.checkin.build.sdkVersion = Option::Some(18);
    checkin_request.version = Option::Some(3);
    checkin_request.fragment = 0;

    checkin_request
}

fn get_checkin_request() -> Result<Request<Body>, Error> {
    let checkin_request = get_checkin_request_payload();
    let uri = "https://android.clients.google.com/checkin";
    let mut buf: Vec<u8> = Vec::new();
    {
        let reference = std::io::Write::by_ref(&mut buf);

        // Adapt reference to `std::io::Write`.
        let mut writer = reference.writer();
        checkin_request.write_message(&mut Writer::new(&mut writer))?;
    }
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuffer"),
        )
        .header(
            CONTENT_LENGTH,
            HeaderValue::from(checkin_request.get_size()),
        )
        .header(
            USER_AGENT,
            HeaderValue::from_static("Android-Checkin/2.0 (vbox86p JLS36G); gzip"),
        )
        .body(hyper::Body::from(buf))?;

    Ok(req)
}

async fn create_gcm_account_future(
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<AndroidAccount, Error> {
    let req = get_checkin_request()?;
    let res = await!(client.request(req))?;
    println!("Response: {:?}", res.status());

    await!(res.into_body().concat2().map(|x| {
        let bytes = x.as_ref();
        let resp = checkin::CheckinResponse::from_reader(
            &mut BytesReader::from_bytes(bytes),
            bytes,
        )?;
        let android_id = match resp.androidId {
            Some(id) => id as i64,
            None => return Err(Error::NoId),
        };
        let security_token = match resp.securityToken {
            Some(token) => token,
            None => return Err(Error::NoToken),
        };
        let acc = AndroidAccount {
            android_id,
            security_token,
        };
        Ok(acc)
    })
    .flatten())
}

async fn request(
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<(), Error> {
    println!("Request new android account");
    let work = await!(create_gcm_account_future(&client))?;
    println!("Account {:?}", work);

    std::thread::sleep(std::time::Duration::from_secs(5));

    await!(get_push_token(&client, work.android_id, work.security_token));

    Ok(())
}

#[derive(Serialize, Debug)]
struct PushTokenRequest<'a> {
    app: &'a str,
    app_ver: &'a str,
    cert: &'a str,
    device: &'a str,
    sender: &'a str,
    #[serde(rename="X-appid")]
    x_appid: &'a str,
    #[serde(rename="X-scope")]
    x_scope: &'a str,
}

async fn get_push_token(client: &Client<HttpsConnector<HttpConnector>>, android_id: i64, security_token: u64) -> Result<(), Error> {
    let uri = "https://android.clients.google.com/c2dm/register3";
    let android_id_str = android_id.to_string();
    let request = PushTokenRequest {
        app: "com.tellm.android.app",
        app_ver: "1001800",
        cert: "a4a8d4d7b09736a0f65596a868cc6fd620920fb0",
        device: &android_id_str,
        sender: "425112442765",
        x_appid: "a5kfH358Kdh", // TODO make this random 11 chars ascii letters and digits
        x_scope: "GCM",
    };
    let body = serde_urlencoded::to_string(&request)?;
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("AidLogin {}:{}", android_id, security_token))?,
        )
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(hyper::Body::from(body))?;

    let res = await!(client.request(req))?;
    println!("Response: {:?}", res.status());
    let res_body = String::from_utf8(await!(res.into_body().concat2())?.as_ref().to_vec()).map_err(|_| Error::NoId)?;
    println!("body: {}", res_body);
    if res_body.starts_with("token=") {
        let token = &res_body[6..];
        println!("token: {}", token);
    }
    Ok(())
}

fn get_login_request<'b>(account: &AndroidAccount) -> mcs::LoginRequest<'b> {
    let mut login_request = mcs::LoginRequest::default();
    login_request.auth_service = Option::Some(mcs::mod_LoginRequest::AuthService::ANDROID_ID);
    login_request.auth_token = Cow::from(account.security_token.to_string());
    login_request.id = Cow::from("android-11");
    login_request.domain = Cow::from("mcs.android.com");
    login_request.device_id = Option::Some(Cow::from(format!("android-{:X}", account.android_id)));
    login_request.resource = Cow::from(account.android_id.to_string());
    login_request.user = login_request.resource.clone();
    login_request.account_id = Option::Some(account.android_id);
    println!("{:?}", login_request);

    login_request
}

/// Reads the next varint encoded u64
fn read_varint64(bytes: &[u8]) -> Result<(u64, usize), Error> {
    let mut i: usize = 0;
    // part0
    let mut b = *bytes.get(i)?;
    if b & 0x80 == 0 {
        return Ok((b as u64, i + 1));
    }
    let mut r0 = (b & 0x7f) as u32;

    i += 1;
    b = *bytes.get(i)?;
    r0 |= ((b & 0x7f) as u32) << 7;
    if b & 0x80 == 0 {
        return Ok((r0 as u64, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r0 |= ((b & 0x7f) as u32) << 14;
    if b & 0x80 == 0 {
        return Ok((r0 as u64, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r0 |= ((b & 0x7f) as u32) << 21;
    if b & 0x80 == 0 {
        return Ok((r0 as u64, i + 1));
    }

    // part1
    i += 1;
    b = *bytes.get(i)?;
    let mut r1 = (b & 0x7f) as u32;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r1 |= ((b & 0x7f) as u32) << 7;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r1 |= ((b & 0x7f) as u32) << 14;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r1 |= ((b & 0x7f) as u32) << 21;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28, i + 1));
    }

    // part2
    i += 1;
    b = *bytes.get(i)?;
    let mut r2 = (b & 0x7f) as u32;
    if b & 0x80 == 0 {
        return Ok(((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56, i + 1));
    }

    i += 1;
    b = *bytes.get(i)?;
    r2 |= (b as u32) << 7;
    if b & 0x80 == 0 {
        return Ok(((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56, i + 1));
    }

    // cannot read more than 10 bytes
    Err(Error::VarInt)
}

async fn read<'a>(account: &'a AndroidAccount) -> Result<(), Error> {
    let mtalk_host = "mtalk.google.com";
    let mtalk_uri = String::from(mtalk_host) + ":5228";
    let server: Vec<_> = mtalk_uri.to_socket_addrs().expect("wrong uri").collect();
    println!("{:?}", server);
    let socket = await!(TcpStream::connect(&server[0]))?;
    let cx = TlsConnector::builder().build().unwrap();
    let cx = tokio_tls::TlsConnector::from(cx);

    let login_request = get_login_request(account);

    let length = login_request.get_size() as u64;

    let stream = await!(
        cx.connect(mtalk_host, socket)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    )?;

    println!("connected");

    let (stream, _) = await!(tokio_io::io::write_all(stream, &[41, 2]))?;
    println!("written number");

    let mut buf: Vec<u8> = Vec::new();
    {
        let reference = std::io::Write::by_ref(&mut buf);

        // Adapt reference to `std::io::Write`.
        let mut writer = reference.writer();
        let proto_writer = &mut Writer::new(&mut writer);
        proto_writer
            .write_varint(length)
            .expect("failed to write length");
        println!("written length");
        login_request.write_message(proto_writer)?;
        println!("written message");
    }

    let (stream, _) = await!(tokio_io::io::write_all(stream, &buf))?;
    println!("read");
    let buf = [0u8; 1];
    let (stream, version) = await!(tokio_io::io::read_exact(stream, buf))?;

    println!("version {:?}", version);
    let mut stream = stream;
    loop {
        println!("reading tag");

        let buf = [0u8; 1];
        let (streams, [response_tag]) = await!(tokio_io::io::read_exact(stream, buf))?;
        stream = streams;

        println!("read tag");

        let buf = [0u8; 10]; // TODO maybe optimize to to read bigger chunks
        let (streams, length_buf) = await!(tokio_io::io::read_exact(stream, buf))?;
        stream = streams;

        println!("read length");

        let (length, consumed_count) = read_varint64(&length_buf)?;

        println!("tag {:?} length {:?}, consumed {:?}", response_tag, length, consumed_count);
        let mut buf = vec![0; length as usize];
        let len = std::cmp::min(length, 10-consumed_count as u64) as usize;
        buf[0..len].copy_from_slice(&length_buf[consumed_count..len+consumed_count]);
        if len < length as usize {
            let remaining = &mut buf[10-consumed_count..];
            let (streams, _) = await!(tokio_io::io::read_exact(stream, remaining))?;
            stream = streams;
        } else {
            panic!("The case that message length smaller 10 is no handled yet!");
        }

        println!("buf {:?}", String::from_utf8_lossy(&buf));

        match response_tag {
            3 => (), // Login
            4 => return Result::Err(Error::NoId), // socket closed by server
            8 => {
                let mut reader = BytesReader::from_bytes(&buf);
                let r = mcs::DataMessageStanza::from_reader(&mut reader, &buf)?;

                println!("stanza {:?}", r);
            },
            _ => (),
        }
    }
}
