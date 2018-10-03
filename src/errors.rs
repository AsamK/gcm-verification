use std::fmt::Display;

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
