use mockito::Server;
use reqwest::{Client, Url};

#[tokio::main]
async fn main() {
    let body = b"Hallo";

    let mut mock_server = Server::new();
    let _mock = mock_server.mock("POST", "/")
            .with_status(200)
            .with_header("Content-Type",  "multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:e14fd72b-9f2d-475f-b581-ee611a495167\"; start=\"<root.message@cxf.apache.org>\"; start-info=\"text/xml\"")
            .with_body(body)
            .create();

    let mut mock_server_url = mock_server.url();
    let server_url = Url::parse(mock_server_url.as_str()).unwrap();

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .post(server_url)
        .body("Hello World")
        .send()
        .await
        .unwrap();
    println!("response: {:?}", response);
    let header = response.headers().get("content-type").unwrap();
    println!("header: {:?}", header);
    assert_eq!("multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:e14fd72b-9f2d-475f-b581-ee611a495167\"; start=\"<root.message@cxf.apache.org>\"; start-info=\"text/xml\"", header)
}
