use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

use super::method::{Method, MethodError};
use super::query_string::{QueryString, QueryValue};

pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParsingError;

    fn try_from(buffer: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsingError::InvalidProtocol)?;

        if protocol != "HTTP/1.1" {
            return Err(ParsingError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        let q = path.find('?');

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        };

        return Ok(Self {
            path,
            query_string,
            method,
        });
    }
}

fn get_next_word(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }

    return None;
}

pub enum ParsingError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParsingError {
    fn get_message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl From<Utf8Error> for ParsingError {
    fn from(_: Utf8Error) -> Self {
        return Self::InvalidEncoding;
    }
}

impl From<MethodError> for ParsingError {
    fn from(_: MethodError) -> Self {
        return Self::InvalidMethod;
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        return write!(f, "{}", self.get_message());
    }
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        return write!(f, "{}", self.get_message());
    }
}
