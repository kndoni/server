use std::str::Utf8Error;
use super::method::{MethodError, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::str;
use std::fmt::{Result as FmtResult, Formatter, Debug};

pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<&'buff str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...Headers...
    fn try_from<'a>(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        /* match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        } */

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string=Some(&path[i+1..]);
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string,
            method,
        })

    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
   
    for (i,c) in request.chars().enumerate()  {
        if c == ' ' || c=='\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { 
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { 
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}