use reqwest::blocking::Client as BlockingClient;
use reqwest::Client as AsyncClient;
use std::error::Error;

pub struct HttpClient {
    pub async_client: AsyncClient,
    pub sync_client: BlockingClient,
}

impl HttpClient {
    pub(crate) fn new() -> Self {
        HttpClient {
            async_client: AsyncClient::new(),
            sync_client: BlockingClient::new(),
        }
    }
}
