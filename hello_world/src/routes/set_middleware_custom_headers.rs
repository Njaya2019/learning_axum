use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};

use super::middleware_custom_message::HeaderMessage;


pub async fn set_middleware_custom_headers<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers.get("message").ok_or_else(|| {
        StatusCode::BAD_REQUEST
    })?;
    let message = message.to_str().map_err(|_error| StatusCode::BAD_REQUEST)?.to_owned();

    let extensions =request.extensions_mut();
    extensions.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}