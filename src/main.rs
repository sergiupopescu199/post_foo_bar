use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// API endpoint url
const POST_URL: &str = "https://httpbin.org/post";

/// General response of the API
#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    pub args: Option<Value>,
    /// raw data being sent to the endpont
    pub data: Option<Value>,
    pub files: Option<Value>,
    /// headers the enpoint received
    pub headers: Headers,
    /// data received from endpoint in json format
    pub json: JsonRes,
    /// ip address of client making HTTP requests to the API endpoint
    pub origin: String,
    /// url of the API endpointr
    pub url: String,
}

/// headers of the API endpoint
#[derive(Debug, Deserialize, Serialize)]
struct Headers {
    /// which MIME type it accepts
    #[serde(rename = "Accept")]
    pub accept: String,
    /// sise of the resource
    #[serde(rename = "Content-Length")]
    pub content_lenght: String,
    /// media type of the resource
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    /// API endpoint DNS name
    #[serde(rename = "Host")]
    pub host: String,
    /// application load balancer request tracer
    #[serde(rename = "X-Amzn-Trace-Id")]
    pub x_amzn_trace_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRes {
    pub foo: String,
}

impl Default for JsonRes {
    fn default() -> Self {
        Self {
            foo: "bar".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // create data to be sent
    let data = JsonRes::default();
    // create a client instance
    let client = Client::new();
    // send data as POST request to the enpoint, it automaticly infers the "Content-Type: application/json" header
    let response = client
        .post(POST_URL)
        // .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;
    // parse the response as json into the general api response struct
    let api_res = response.json::<ApiResponse>().await?;
    // print headers struct
    println!("{:?}", api_res.headers);
    // print json struct
    println!("{:?}", api_res.json);
    Ok(())
}
