use serde::Serialize;

use crate::{
    apis::{self, transactions_api::EncodeSubmissionError},
    client::{CallOptions, HttpProvider},
};

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
}

#[derive(Debug, Clone)]
pub struct Client {}
pub struct Response {
    status: u16,
    body: String,
}

#[derive(Debug, Clone)]
pub struct StatusCode(u16);
impl StatusCode {
    pub fn is_server_error(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
    pub fn is_client_error(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }
}
impl Response {
    pub fn new(status: u16, body: String) -> Self {
        Self { status, body }
    }
    pub fn status(&self) -> StatusCode {
        StatusCode(self.status)
    }
    pub async fn text<E>(&self) -> Result<String, apis::Error<E>> {
        Ok(self.body.clone())
    }
}

impl Client {
    pub fn request(&self, method: HttpMethod, url: &str) -> RequestBuilder {
        RequestBuilder::new(method, url)
    }
    pub async fn execute<E>(&self, request: Request) -> Result<Response, apis::Error<E>> {
        Ok(HttpProvider::new()
            .send(request, CallOptions::default())
            .await)
    }
}

pub struct RequestBuilder {
    method: HttpMethod,
    url: String,
    queries: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    body: Option<Vec<u8>>,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    method: HttpMethod,
}

impl Request {
    pub fn body(&self) -> Option<Vec<u8>> {
        self.body.clone()
    }
    pub fn query(&self) -> Vec<(String, String)> {
        self.query.clone()
    }
    pub fn headers(&self) -> Vec<(String, String)> {
        self.headers.clone()
    }
    pub fn method(&self) -> HttpMethod {
        self.method.clone()
    }
}

impl RequestBuilder {
    pub fn new(method: HttpMethod, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            queries: Vec::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn query(self, params: &[(&str, &str)]) -> Self {
        let mut queries = self.queries;
        for (key, value) in params {
            queries.push((key.to_string(), value.to_string()));
        }
        Self { queries, ..self }
    }

    pub fn header(self, key: String, value: String) -> Self {
        let mut headers = self.headers;
        headers.push((key, value.to_string()));
        Self { headers, ..self }
    }

    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        let body = serde_json::to_string(json).unwrap();
        Self {
            body: Some(body.as_bytes().to_vec()),
            ..self
        }
    }

    pub fn build<E>(&self) -> Result<Request, apis::Error<E>> {
        Ok(Request {
            body: self.body.clone(),
            query: self
                .queries
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            headers: self.headers.clone(),
            method: self.method.clone(),
            url: self.url.clone(),
        })
    }
}
