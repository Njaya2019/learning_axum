use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage( pub String);

pub async fn middleware_custom_message(Extension(message): Extension<HeaderMessage>) -> String{
    message.0
}