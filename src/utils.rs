use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            message: "success".to_string(),
            data,
        }
    }
    
    pub fn error(msg: &str) -> Self where T: Default {
         Self {
            status: 500,
            message: msg.to_string(),
            data: T::default(),
        }
    }
}
