use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Msg(&'static str),
    NoId,
    NoToken,
    VarInt,
    Hyper(hyper::Error),
    Http(http::Error),
    HttpHeader(http::header::InvalidHeaderValue),
    SerdeUrl(serde_urlencoded::ser::Error),
    SerdeDe(serde::de::value::Error),
    SerdeJ(serde_json::Error),
    Addr(std::net::AddrParseError),
    Protobuf(quick_protobuf::errors::Error),
    Io(std::io::Error),
    InvalidUri(http::uri::InvalidUri),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Msg(msg) => write!(f, "{}", msg),
            Error::NoId => write!(f, "Response contains no android id"),
            Error::NoToken => write!(f, "Response contains no token"),
            _ => Ok(()),
        }
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
        Error::SerdeUrl(err)
    }
}

impl From<serde::de::value::Error> for Error {
    fn from(err: serde::de::value::Error) -> Error {
        Error::SerdeDe(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJ(err)
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

impl From<http::uri::InvalidUri> for Error {
    fn from(err: http::uri::InvalidUri) -> Error {
        Error::InvalidUri(err)
    }
}
