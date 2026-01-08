use serde::Serialize;
use sha2::{Sha256, Digest};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "ok".to_string(),
            data,
        }
    }
    
    pub fn error(msg: &str) -> Self where T: Default {
         Self {
            code: 500,
            message: msg.to_string(),
            data: T::default(),
        }
    }
}

pub fn encrypt_password(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}
