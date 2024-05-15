// Copyright 2024 the Cartero authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use isahc::http::header::{InvalidHeaderName, InvalidHeaderValue};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl TryFrom<&str> for RequestMethod {
    type Error = RequestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(RequestMethod::Get),
            "post" => Ok(RequestMethod::Post),
            "put" => Ok(RequestMethod::Put),
            "patch" => Ok(RequestMethod::Patch),
            "delete" => Ok(RequestMethod::Delete),
            "options" => Ok(RequestMethod::Options),
            "head" => Ok(RequestMethod::Head),
            _ => Err(RequestError::InvalidHttpVerb),
        }
    }
}

impl From<RequestMethod> for &str {
    fn from(val: RequestMethod) -> Self {
        match val {
            RequestMethod::Get => "GET",
            RequestMethod::Post => "POST",
            RequestMethod::Put => "PUT",
            RequestMethod::Patch => "PATCH",
            RequestMethod::Delete => "DELETE",
            RequestMethod::Head => "HEAD",
            RequestMethod::Options => "OPTIONS",
        }
    }
}

impl From<RequestMethod> for String {
    fn from(value: RequestMethod) -> String {
        let string: &str = value.into();
        String::from(string)
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: RequestMethod,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(
        url: String,
        method: RequestMethod,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            url,
            method,
            headers,
            body,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub duration: u64, // miliseconds
    pub size: u64,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn body_as_str(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Illegal HTTP verb")]
    InvalidHttpVerb,

    #[error("Invalid headers state")]
    InvalidHeaders,

    #[error("Illegal header")]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error("Illegal header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Request error")]
    NetworkError(#[from] isahc::error::Error),

    #[error("HTTP error")]
    HttpError(#[from] isahc::http::Error),

    #[error("Unknown I/O error")]
    IOError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_convert_str_to_method() {
        assert!(RequestMethod::try_from("GET").is_ok_and(|x| x == RequestMethod::Get));
        assert!(RequestMethod::try_from("post").is_ok_and(|x| x == RequestMethod::Post));
        assert!(RequestMethod::try_from("Patch").is_ok_and(|x| x == RequestMethod::Patch));
        assert!(RequestMethod::try_from("Juan").is_err());
    }
}
