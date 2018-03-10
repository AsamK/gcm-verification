extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate quick_protobuf;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use hyper::{Method, Request};
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::header::{ContentLength, ContentType};
use hyper::header::UserAgent;
use hyper_tls::HttpsConnector;
use protos::checkin;
use protos::mcs;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};
use std::borrow::Cow;
use std::default::Default;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

mod protos;

fn main() {
    println!("Hello, world!");
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    //    let client = Client::new(&core.handle());
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    match request(&mut core, client) {
        Ok(work) => {
            println!("Ok: {:?}", work);

            match read(&mut core, &handle, &work) {
                Ok(work) => println!("Ok: {:?}", work),
                Err(foo) => println!("Err: {:?}", foo),
            }
        }
        Err(foo) => println!("Err: {:?}", foo),
    }
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
    Hyper(hyper::Error),
    Uri(hyper::error::UriError),
    Addr(std::net::AddrParseError),
    Protobuf(quick_protobuf::errors::Error),
    Io(std::io::Error),
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Error {
        Error::Addr(err)
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(err: hyper::error::UriError) -> Error {
        Error::Uri(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
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

fn request(
    core: &mut Core,
    client: Client<HttpsConnector<HttpConnector>>,
) -> Result<AndroidAccount, Error> {
    let mut checkin_request = checkin::CheckinRequest::default();
    checkin_request.checkin.build.sdkVersion = Option::Some(18);
    checkin_request.version = Option::Some(3);
    checkin_request.fragment = 0;
    let uri = "https://android.clients.google.com/checkin".parse()?;
    //    let uri = "https://google.com".parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut()
        .set(ContentType("application/x-protobuffer".parse().unwrap()));
    req.headers_mut()
        .set(ContentLength(checkin_request.get_size() as u64));
    //    req.headers_mut().set(AcceptEncoding(vec![qitem(Encoding::Gzip)]));
    req.headers_mut()
        .set(UserAgent::new("Android-Checkin/2.0 (vbox86p JLS36G); gzip"));

    let mut buf: Vec<u8> = Vec::new();
    checkin_request.write_message(&mut Writer::new(&mut buf))?;
    req.set_body(buf);
    let work = client
        .request(req)
        .map(|res| -> Result<AndroidAccount, Error> {
            //    let work = client.get(uri).and_then(|res| {
            println!("Response: {:?}", res.status());

            let total = res.body().concat2().wait().unwrap();
            let resp = checkin::CheckinResponse::from_reader(
                &mut BytesReader::from_bytes(total.as_ref()),
                total.as_ref(),
            ).unwrap();
            let android_id = match resp.androidId {
                Some(id) => id as i64,
                None => return Err(Error::NoId),
            };
            let security_token = match resp.securityToken {
                Some(token) => token,
                None => return Err(Error::NoToken),
            };
            let acc = AndroidAccount {
                android_id: android_id,
                security_token: security_token,
            };
            println!("response: {:?} - {:?}", resp, acc);
            //            io::stdout()
            //                .write_all(&chunk)
            //                .map_err(From::from)
            Ok(acc)
        });
    match core.run(work) {
        Ok(work) => {
            println!("Ok: {:?}", work);
            work
        }
        Err(foo) => {
            println!("Err: {:?}", foo);
            Err(std::convert::From::from(foo))
        }
    }
}

fn read(core: &mut Core, handle: &Handle, account: &AndroidAccount) -> Result<&'static str, Error> {
    let mtalk_uri = &"mtalk.google.com:5228";
    let server: Vec<_> = mtalk_uri.to_socket_addrs().expect("wrong uri").collect();
    println!("{:?}", server);
    let connection = TcpStream::connect(&server[0], &handle);

    let mut login_request = mcs::LoginRequest::default();
    login_request.auth_service = Option::Some(mcs::mod_LoginRequest::AuthService::ANDROID_ID);
    login_request.auth_token = Cow::from(account.security_token.to_string());
    login_request.id = Cow::from("android-11");
    login_request.domain = Cow::from("mcs.android.com");
    login_request.device_id = Option::Some(Cow::from(format!("android-{:X}", account.android_id)));
    login_request.resource = Cow::from(account.android_id.to_string());
    login_request.user = Cow::from(account.android_id.to_string());
    login_request.account_id = Option::Some(account.android_id);

    let length = login_request.get_size() as u64;

    let client = connection.and_then(move |mut stream| {
        stream.write_all(&[41, 2])?;
        {
            let writer = &mut Writer::new(&mut stream);
            writer.write_varint(length);
            login_request.write_message(writer);
        }

        let mut buf = [0; 1];
        let version = stream.read_exact(&mut buf);
        println!("verison {:?}", version);
        //        let mut buf = BytesMut::with_capacity(1000);
        //        stream
        //            .read_buf(&mut buf)
        //            .map(|buf| print!("Buffer {:?}", buf))
        //            .map_err(|e| eprintln!("Error: {}", e));
        Ok(())
    });

    match core.run(client) {
        Ok(work) => println!("Ok: {:?}", work),
        Err(err) => println!("Err: {:?}", err),
    }

    Ok("foo")
}
