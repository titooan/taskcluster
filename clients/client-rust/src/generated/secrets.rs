#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/* THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, ClientBuilder, Credentials, Retry};
use anyhow::Error;
use serde_json::Value;
use std::time::Duration;
use crate::util::urlencode;

/// Secrets Service
///
/// The secrets service provides a simple key/value store for small bits of secret
/// data.  Access is limited by scopes, so values can be considered secret from
/// those who do not have the relevant scopes.
///
/// Secrets also have an expiration date, and once a secret has expired it can no
/// longer be read.  This is useful for short-term secrets such as a temporary
/// service credential or a one-time signing key.
pub struct Secrets (Client);

#[allow(non_snake_case)]
impl Secrets {
    /// Create a new undefined instance, based on the given client.
    pub fn new<CB: Into<ClientBuilder>>(client_builder: CB) -> Result<Self, Error> {
        Ok(Self(client_builder
            .into()
            .path_prefix("api/secrets/v1/")
            .build()?))
    }

    /// Ping Server
    /// 
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::ping_details();
        let body = None;
        let resp = self.0.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the ping endpoint
    pub fn ping_url(&self) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.0.make_url(path, query)
    }

    /// Generate a signed URL for the ping endpoint
    pub fn ping_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.0.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for ping
    fn ping_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "ping";
        let query = None;

        (path, query)
    }

    /// Set Secret
    /// 
    /// Set the secret associated with some key.  If the secret already exists, it is
    /// updated instead.
    pub async fn set(&self, name: &str, payload: &Value) -> Result<(), Error> {
        let method = "PUT";
        let (path, query) = Self::set_details(name);
        let body = Some(payload);
        let resp = self.0.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Determine the HTTP request details for set
    fn set_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("secret/{}", urlencode(name));
        let query = None;

        (path, query)
    }

    /// Delete Secret
    /// 
    /// Delete the secret associated with some key. It will succeed whether or not the secret exists
    pub async fn remove(&self, name: &str) -> Result<(), Error> {
        let method = "DELETE";
        let (path, query) = Self::remove_details(name);
        let body = None;
        let resp = self.0.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Determine the HTTP request details for remove
    fn remove_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("secret/{}", urlencode(name));
        let query = None;

        (path, query)
    }

    /// Read Secret
    /// 
    /// Read the secret associated with some key.  If the secret has recently
    /// expired, the response code 410 is returned.  If the caller lacks the
    /// scope necessary to get the secret, the call will fail with a 403 code
    /// regardless of whether the secret exists.
    pub async fn get(&self, name: &str) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::get_details(name);
        let body = None;
        let resp = self.0.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the get endpoint
    pub fn get_url(&self, name: &str) -> Result<String, Error> {
        let (path, query) = Self::get_details(name);
        self.0.make_url(&path, query)
    }

    /// Generate a signed URL for the get endpoint
    pub fn get_signed_url(&self, name: &str, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::get_details(name);
        self.0.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for get
    fn get_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("secret/{}", urlencode(name));
        let query = None;

        (path, query)
    }

    /// List Secrets
    /// 
    /// List the names of all secrets.
    /// 
    /// By default this end-point will try to return up to 1000 secret names in one
    /// request. But it **may return less**, even if more tasks are available.
    /// It may also return a `continuationToken` even though there are no more
    /// results. However, you can only be sure to have seen all results if you
    /// keep calling `listTaskGroup` with the last `continuationToken` until you
    /// get a result without a `continuationToken`.
    /// 
    /// If you are not interested in listing all the members at once, you may
    /// use the query-string option `limit` to return fewer.
    pub async fn list(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::list_details(continuationToken, limit);
        let body = None;
        let resp = self.0.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the list endpoint
    pub fn list_url(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::list_details(continuationToken, limit);
        self.0.make_url(path, query)
    }

    /// Generate a signed URL for the list endpoint
    pub fn list_signed_url(&self, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::list_details(continuationToken, limit);
        self.0.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for list
    fn list_details<'a>(continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "secrets";
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }
}