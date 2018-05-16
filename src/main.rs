extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate quick_protobuf;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use hyper::{Body, Method, Request};
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::header::{ContentLength, ContentType};
use hyper::header::UserAgent;
use hyper_tls::HttpsConnector;
use protos::checkin;
use protos::mcs;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Reader, Writer};
use std::borrow::Cow;
use std::default::Default;
use std::fmt::Display;
use std::net::ToSocketAddrs;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

mod protos;

fn main() {
    println!("Starting Jodel GCM verification server");
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    //    let client = Client::new(&core.handle());
    let client = Client::configure()
        .keep_alive(false)
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    match request(&mut core, client) {
        Ok(work) => {
            println!("Ok: {:?}", work);
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

fn get_checkin_request_payload<'a>() -> checkin::CheckinRequest<'a> {
    let mut checkin_request = checkin::CheckinRequest::default();
    checkin_request.checkin.build.sdkVersion = Option::Some(18);
    checkin_request.version = Option::Some(3);
    checkin_request.fragment = 0;

    checkin_request
}

fn get_checkin_request() -> Result<Request<Body>, Error> {
    let checkin_request = get_checkin_request_payload();
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
    Ok(req)
}

fn create_gcm_account_future(
    client: Client<HttpsConnector<HttpConnector>>,
) -> Result<impl Future<Item=Result<AndroidAccount, Error>, Error=impl std::error::Error>, Error> {
    let req = get_checkin_request()?;
    let work = client
        .request(req)
        .map(|res| {
            println!("Response: {:?}", res.status());

            res.body()
                .concat2()
//                .fold(Vec::new(), |mut acc, chunk| {
//                    acc.extend_from_slice(&*chunk);
//                    futures::future::ok::<_, hyper::Error>(acc)
//                })
                .map(|x| {
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
        }).flatten();
    Ok(work)
}

fn request(
    core: &mut Core,
    client: Client<HttpsConnector<HttpConnector>>,
) -> Result<(), Box<std::error::Error>> {
    let handle = core.handle();
    let work = create_gcm_account_future(client)?
        .map_err(|err| -> Box<std::error::Error> { Box::new(err) })
        .and_then(|work| {
            match work {
                Ok(work) => {
                    println!("Account {:?}", work);
                    read(&handle, &work)
                        .map_err(|err| -> Box<std::error::Error> { Box::new(err) })
                }

                Err(foo) => panic!("Err: {:?}", foo),
            }
//            match core.run(client) {
//                Ok(work) => println!("Okf: {:?}", work),
//                Err(err) => println!("Err: {:?}", err),
//            }
//
//            Ok("foo")
        });


    match core.run(work) {
        Ok(work) => {
            println!("Ok: {:?}", work);
//            read(&handle, &work.unwrap());
            Ok(())
        }
        Err(foo) => {
            println!("Err: {:?}", foo);
            Err(std::convert::From::from(foo))
        }
    }
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

fn read(handle: &Handle, account: &AndroidAccount) -> impl Future<Item=(), Error=impl std::error::Error> {
    let mtalk_uri = &"mtalk.google.com:5228";
    let server: Vec<_> = mtalk_uri.to_socket_addrs().expect("wrong uri").collect();
    println!("{:?}", server);
    let connection = TcpStream::connect(&server[0], &handle);

    let login_request = get_login_request(account);

    let length = login_request.get_size() as u64;

    let client = connection
        .and_then(move |stream| {
            println!("connected");
            tokio_io::io::write_all(stream, &[41, 2])
        })
        .and_then(move |(mut stream, _)| {
//            stream.write_all(&[41, 2]);
            println!("written number");
            {
                let writer = &mut Writer::new(&mut stream);
                writer.write_varint(length).expect("failed to write length");
                println!("written length");
                login_request.write_message(writer).expect("failed to write message");
            }
            println!("read");
            let buf = [0u8; 1];
            tokio_io::io::read_exact(stream, buf)
        })
        .and_then(|(stream, tag)| {
            println!("verison {:?}", tag);
            let buf = Vec::new();
            tokio_io::io::read_to_end(stream, buf)
        })
        .and_then(|(stream, buf)| {
            let mut reader = BytesReader::from_bytes(&buf);
//            let r = mcs::DataMessageStanza::from_reader(&mut reader, &buf).expect("Cannot read FooBar");

            while !reader.is_eof() {
                println!("verison {:?}", String::from_utf8_lossy(&buf));
                let foobar: mcs::DataMessageStanza = reader.read_message(&buf).expect("not working");
                println!("data message {:?}", foobar);
            }
            futures::future::ok(())
        })
//        .map_err(|err| -> () { () })

    //        let mut buf = BytesMut::with_capacity(1000);
    //        stream
    //            .read_buf(&mut buf)
    //            .map(|buf| print!("Buffer {:?}", buf))
    //            .map_err(|e| eprintln!("Error: {}", e));
    ;
//    handle.spawn(client)
    client
}
