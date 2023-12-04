use reqwest::Client as AsyncClient;

#[derive(Debug)]
pub struct HttpClient {
    pub async_client: AsyncClient,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            async_client: AsyncClient::new(),
        }
    }
}
