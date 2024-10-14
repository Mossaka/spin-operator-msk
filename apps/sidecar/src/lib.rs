use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_sidecar(req: Request) -> anyhow::Result<impl IntoResponse> {
     // Create the outbound request object
     let request = Request::builder()
     .method(req.method().clone())
     .uri("http://127.0.0.1:88")
     .build();

    // Send the request and await the response
    let response: Response = spin_sdk::http::send(request).await?;

    Ok(response)
}
