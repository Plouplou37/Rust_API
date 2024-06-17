use {
    actix_web::HttpResponse,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct NotFoundMessage {
    message: String,
}

impl NotFoundMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

// Enum representing different response types with a generic T.
pub enum ResponseType<T> {
    Ok(T),       // Successful response with payload (e.g., Miner or Wallet)
    NotFound(T), // Not found response with payload (e.g., NotFoundMessage)
    Created(T),  // Created response with payload (e.g., Miner or Wallet)
}

// Implement methods for ResponseType with a constraint on T to be serializable.
impl<T: Serialize> ResponseType<T> {
    pub fn get_response(&self) -> HttpResponse {
        match self {
            ResponseType::Ok(payload) => HttpResponse::Ok()
                .content_type("application/json")
                .json(payload),
            ResponseType::NotFound(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(message),
            ResponseType::Created(payload) => HttpResponse::Created()
                .content_type("application/json")
                .json(payload),
        }
    }
}
