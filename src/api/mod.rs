pub mod auth;
pub mod goods;
pub mod menu_tree;
pub mod metrics;
pub mod router;
pub mod story;
pub mod system;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "成功".to_string(),
            data,
        }
    }

    #[allow(dead_code)]
    pub fn error(code: i32, message: impl Into<String>, data: T) -> Self {
        Self {
            code,
            message: message.into(),
            data,
        }
    }
}
